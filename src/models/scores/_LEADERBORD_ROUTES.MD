# These API routes are designed for the Website and will make things easier for me on the Front end

## Routes

[ ] - Leaderboard with usernames & table info
[ ] - Account Settings
[ ] - Profile Data
[ ]

## Files

### number_of_records.rs

Returns the number of records meeting the filter conditions passed to it.

### leaderboard_web.rs

Takes in a set of filtering and ordering conditions and returns a set of paginated? results
as well as the total number of records meeting the filter conditions.

### user_position.rs

Takes in a user_id/username and returns the position in the global leaderboard.
Another route can get the user's position depending on filtering conditions.
