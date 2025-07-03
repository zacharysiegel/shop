# TODO

## Images

1. Serve static images from NGINX
    * [done] Perhaps this easily leads to serving the other static files as well
2. Implement image uploads through `frontend`/`inventory`.
    * [done] Simple file upload UI
    * [done] Create `item_image` record in the database.
    * [done] Expect the image URI to be `[images]/[item_image_id]`
        * [done] Write a small "uri generator" method
    * [done] Store the image file at the expected URI.
    * [done] Conform upload button to raw payload expectation
3. Upload images to eBay
    * [done] Get all images associated with a given item. 
        * `select * from item_image where item_id = [id]`
    * [done] Invoke [UploadSiteHostedPictures](https://developer.ebay.com/Devzone/XML/docs/Reference/eBay/UploadSiteHostedPictures.html)
    * Add ebay sync button per item (or image if that is preferred)
4. Delete images
   * Delete button with mini form per image
   * Endpoint
   * Delete image file
   * Delete item_image record
   * Refresh image list

## eBay webhook

[Platform Notifications](https://developer.ebay.com/api-docs/static/platform-notifications-landing.html)
