# TODO

## Images

1. Serve static images from NGINX
    * [done] Perhaps this easily leads to serving the other static files as well
2. Implement image uploads through `frontend`/`inventory`.
    * Simple file upload UI
    * Create `item_image` record in the database.
    * Expect the image URI to be `[images]/[item_image_id]`
        * Write a small "uri generator" method
    * Store the image file at the expected URI.
3. Upload images to eBay
    * Get all images associated with a given item. 
        * `select * from item_image where item_id = [id]`
    * Invoke [UploadSiteHostedPictures](https://developer.ebay.com/Devzone/XML/docs/Reference/eBay/UploadSiteHostedPictures.html)

## eBay webhook

[Platform Notifications](https://developer.ebay.com/api-docs/static/platform-notifications-landing.html)

