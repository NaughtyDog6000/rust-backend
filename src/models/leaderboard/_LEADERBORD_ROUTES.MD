# These API routes are designed for the Website and will make things easier for me on the Front end

# Routes

[ ] - Leaderboard with usernames & table info
[ ] - Account Settings
[ ] - Profile Data
[ ]

# Files

<!-- NUMBER OF RECORDS MATCHING FILTERS / IN DB -->
## number_of_records.rs

Returns the number of records meeting the filter conditions passed to it.

Returns the number of records possessed by a user

<!-- LEADERBOARD RECORDS ROUTE -->
## leaderboard.rs

Takes in a set of filtering and ordering conditions and returns a set of paginated? results
as well as the total number of records meeting the filter conditions.

<!-- USER POSITION IN LEADERBOARD ROUTE -->
## user_position.rs

### Requires Auth

Takes in a user_id/username and returns the position in the global leaderboard.
Another route can get the user's position depending on filtering conditions.

<!-- UPLOAD ROUTE -->
## upload_score.rs

<span style="color: #FF5555; font-size: large">Requires Auth</span>.

### Description/Info

This route is used by the game in order to upload records to the database where it can be served on the leaderboard and your profile.

### Example(s)

### Inputs

```Javascript
    {
        "score": 123,
        "gamemode": "Default",
        "epoch_game_start_time": 12345,
        "epoch_game_end_time": 123456
    }
```

### Responses

Success:

```Javascript
{   
    "response": "score record creation successful"
}
```

Repeated Upload of same game:

```Javascript
{
    "error": "this score has already been uploaded"
}
```
