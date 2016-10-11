# v0.2.0
* addext
  * better reporting.
  * cmd option to turn on file renaming. Otherwise, just finds jpegs without extensions.
* finddups
  * option to delete the newest copies of duplicate files (with a suboption to report only)

# v0.0.1
* Basic finddups subcommand working
  * Compares sha256 hash of all files in supplied subdirs and "opens" the dups.
  * Opening can involve printing the paths, opening with Preview, or nothing.
* Basic addext subcommand working
  * find jpegs with missing file extension and (optionally) add the .jpg file extension.
