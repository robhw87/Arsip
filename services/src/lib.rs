pub mod error;
pub mod scan;
/*
* Services :
*   - Scan : looping the root dir to parse it into validate content
*   - commit : update insert data from scan into database
*   - archive: change status of database into archive
*   - delete: delete data from database
*   - list: show data from databases
*   - compare: compare current data with databases
*   - show: showing 1 row database base on user search slug or slug aliases
*   - restore: create md file base on databases
*   - purge: delete all data in database
*   - tags : tags command in arsip
*   - catogery : category command in arsip
* Domain :
*   - validate_content: validating each md file content to be as cms structure
*   - compare_content : comparing current data into databases
*   - compare_tags: check tags in database and the current scan result
*   - compare_categories: check catogery in database and the current scan result
* Adapters:
*/
