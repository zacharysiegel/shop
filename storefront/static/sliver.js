/**
 * @module sliver
 * @description A truly minimal, opinionated UI component framework for operating directly with the browser.
 *
 * @author Zachary Siegel
 * @version 0.1.0
 *
 * @example
 * // Definition
 * const Counter = component()
 *   .properties({ count: 0 })
 *   .factory(({ fragment, properties, add_callback }) => {
 *     let count = properties.count;
 *
 *     const span = document.createElement('span');
 *     span.textContent = properties.count;
 *
 *     add_callback('increment', () => {
 *       count += 1;
 *       span.textContent = count;
 *     });
 *
 *     fragment.appendChild(span);
 *   })
 *   .build();
 *
 * // Instantiation
 * const counter = Counter();
 * const btn = document.createElement('button');
 * btn.textContent = '+';
 * btn.onclick = counter.callbacks.increment;
 *
 * document.body.appendChild(counter.element);
 * document.body.appendChild(btn);
 *
 * // Or as Web Component
 * component().properties({ count: 0 }).factory(...).define('my-counter');
 * // <my-counter count="5"></my-counter>
 * // <button onclick="document.querySelector('my-counter').increment()">+</button>
 */

/**
 * @typedef {Object} ComponentFactoryProperties
 * @readonly
 * @property {DocumentFragment} fragment - The document fragment root of the component
 * @property {Object} properties - Component properties merged with defaults
 * @property {function(string, function): void} add_callback - Register a named callback function
 */

/**
 * @callback ComponentFactory
 * @param {ComponentFactoryProperties} params - Component creation parameters
 * @returns {void}
 */

/**
 * @typedef {Object} ComponentInstance
 * @property {DocumentFragment} element - The component's document fragment root
 * @property {Object.<string, function>} callbacks - Object containing registered callbacks
 */

/**
 * @callback Component
 * @param {Object} [properties={}] - Properties to pass to the component
 * @returns {ComponentInstance} The component instance
 */

/**
 * @typedef {Object} ComponentBuilder
 * @property {function(Object): ComponentBuilder} properties - {@link ComponentBuilder#properties}
 * @property {function(boolean=): ComponentBuilder} shadow - {@link ComponentBuilder#shadow}
 * @property {function(ComponentFactory): ComponentBuilder} factory - {@link ComponentBuilder#factory}
 * @property {function(string, Object=): ComponentBuilder} define - {@link ComponentBuilder#define}
 * @property {function(): Component} build - {@link ComponentBuilder#build}
 */

/**
 * Initialize a component builder.
 * @returns {ComponentBuilder} A new component builder
 */
function component() {
    /** @type {Object.<string, any>} */
    let property_defaults = {};

    /** @type {boolean} */
    let use_shadow = false;

    /** @type {(ComponentFactory|null)} */
    let factory_function = null;

    /** @type {(Component|null)} */
    let _component_fn = null;

    /** @type {ComponentBuilder} */
    const builder = {
        /**
         * Set default property values for the component
         * @memberof ComponentBuilder
         * @param {Object.<string, any>} properties
         * @returns {ComponentBuilder}
         */
        properties: (properties) => {
            if (_component_fn) throw new Error("Cannot modify properties after component is built");

            property_defaults = {...property_defaults, ...properties};
            return builder;
        },

        /**
         * Enable or disable shadow DOM for web components.
         * Disabled by default.
         * This property has no effect if {@link ComponentBuilder#define} is not also used.
         * @memberof ComponentBuilder
         * @param {boolean} enabled
         * @returns {ComponentBuilder}
         */
        shadow: (enabled) => {
            if (_component_fn) throw new Error("Cannot modify shadow DOM setting after component is built");

            use_shadow = enabled;
            return builder;
        },

        /**
         * Set the factory function which creates component instances
         * @memberof ComponentBuilder
         * @param {ComponentFactory} fn
         * @returns {ComponentBuilder}
         */
        factory: (fn) => {
            if (_component_fn) throw new Error("Cannot modify factory after component is built");

            factory_function = fn;
            return builder;
        },

        /**
         * Build and return the component.
         * This will always return the same reference if called multiple times.
         * @memberof ComponentBuilder
         * @returns {Component}
         */
        build: () => {
            if (_component_fn) {
                return _component_fn;
            }

            _component_fn = (properties = {}) => {
                const merged_properties = {...property_defaults, ...properties};
                const fragment = document.createDocumentFragment();
                const callbacks = {};

                factory_function(Object.freeze({
                    fragment,
                    properties: merged_properties,
                    add_callback: (name, fn) => {
                        callbacks[name] = fn;
                    },
                }));

                return {
                    element: fragment,
                    callbacks: callbacks,
                };
            };
            return _component_fn;
        },

        /**
         * Define the component as a custom element (Web Component).
         * {@link https://developer.mozilla.org/en-US/docs/Web/API/Web_components}
         * @memberof ComponentBuilder
         * @param {string} tag_name
         * @param {ElementDefinitionOptions} [options={}] - The same options as are available on {@link CustomElementRegistry.define}
         * @returns {ComponentBuilder}
         */
        define: (tag_name, options = {}) => {
            // noinspection JSClosureCompilerSyntax; JetBrains warns about unimplemented HTMLElement methods.
            /**
             * Custom element class
             * @extends HTMLElement
             */
            class CustomElement extends HTMLElement {
                constructor() {
                    super();

                    /** @type {Object.<string, function>} */
                    this.__callbacks = {};

                    /** @type {ComponentInstance|null} */
                    this.__instance = null;
                }

                /**
                 * Called when the element is connected to the DOM
                 * @private
                 */
                connectedCallback() {
                    if (this.__instance) return;

                    const component_properties = {};
                    for (const key of Object.getOwnPropertyNames(property_defaults)) {
                        const attr = this.getAttribute(key);
                        component_properties[key] = attr !== null ? attr : property_defaults[key];
                    }

                    const component_fn = builder.build();
                    this.__instance = component_fn(component_properties);

                    // Mount fragment contents to shadow DOM or directly
                    const root = use_shadow
                        ? this.attachShadow({mode: "open"})
                        : this;
                    root.appendChild(this.__instance.element);

                    // Expose callbacks as element methods
                    this.__callbacks = this.__instance.callbacks;
                    Object.keys(this.__callbacks).forEach(name => {
                        this[name] = this.__callbacks[name];
                    });
                }

                /**
                 * Get list of observed attributes
                 * @returns {string[]} Array of attribute names to observe
                 * @static
                 */
                static get observedAttributes() {
                    return Object.keys(property_defaults);
                }

                /**
                 * Called when an observed attribute changes
                 * @param {string} name - Attribute name
                 * @param {string|null} old_value - Previous value
                 * @param {string|null} new_value - New value
                 * @private
                 */
                attributeChangedCallback(name, old_value, new_value) {
                    if (this.__callbacks[`set_${name}`]) {
                        this.__callbacks[`set_${name}`](new_value);
                    }
                }
            }

            window.customElements.define(tag_name, CustomElement, options);
            return builder;
        }
    };

    return builder;
}

export {component};
