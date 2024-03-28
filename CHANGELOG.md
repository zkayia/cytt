
# 2.0.1

### Fixed
- no subject found when classroom is missing
- switched to NaiveDateTime to avoid timezone shenanigans

### Miscellaneous
- updated config in readme
- satisfied the almighty clippy
- applied cargo fmt

# 2.0.0

### Fixed
- made login required on a per group basis 

### Miscellaneous
- updated website link in readme

# 1.0.4

### Fixed
- moved events duplicating in db

### Miscellaneous
- changed fetch parameters to be more consistant

# 1.0.3

### Fixed
- png output not displaying next week on weekends

### Miscellaneous
- moved version handling out of cargo to avoid the docker layer cache being invalidated on new releases

# 1.0.2

### Fixed
- public directory being created in the data path (although not used)
- all groups being displayed at once on initial load
- some elements being visible through the header

### Miscellaneous
- improve responsiveness on narrow devices (<358px)

# 1.0.1

### Features
- optional display name for groups
- independant public and data folders

### Fixed
- group selector value not updated when loading last selection from localStorage
- selection of invalid group when group list changes and loading last selection from localStorage

### Miscellaneous
- add group name to png outputs
- reworked home page header 
- reduced log density of calendar updates
- group directories are created on startup instead of on each calendar update

# 1.0.0

Initial version
