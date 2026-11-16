pub mod content_validation;
pub mod scan;
/*
 # Arsip CMS

 Arsip CMS is a simple CLI CMS.
 This CMS is build for learning TyDD methods and also clean / hexagonal architecture

 ## Planning

 ### Desired Features

 1. Work in CLI and use as API for website
 2. reads a directory (specified in CMS config or directly in the command)
 3. what it reads, is md files.
 4. the md files is formatted with metadata at the top (format in YAML with  --- delimiter) and
    the content
 5. Scan command will scan the dir for each md files it content. Then give result with status
    waiting user order
 6. Push command will push the result of scanning to database and also redis
 7. Sync command will directly scan and push the data
 8. The source of truth will be the databases. So the scan and pushing will only affecting the
    data that the dir contains.which means that if a files or data in database is not available inside the dir, it will not change anything to the data.
 9. Pull command creating a md file base on the data in databases
 10. publish command changing status of a row data
 11. ls command show list of all data (without the content only metadata) in databases
 12. archive command to archieving status to data
 13. drop command to delete a data

 ### output design

 Metadata
 Content

 ### process

 Slug
 ContentStatus
 ContentLanguage
 ScannedMetadata
 ScannedContent

 ### input

 scan_dir


Parsing data

*/
