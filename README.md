# rust-backend

this is my first project in rust which will be to make a backend for compsci project
it will include: **Accounts**, Player **Achievements** (if I get round to it), **Leaderboards**, Potentially a way to **verify/simulate a game** when its uploaded to make sure that score values were not modified, **Friends** (am working on aquireing one of these to be able to test this concept), finally if I can be bothered it would store telemetry data to be able to make informed changes (what the most/least used things are, average game length, most difficult sections etc)

## Things I need to do

**Testing** - [] automated testing  
**Routes** - [] different routes for different things  
**DB** - [] Atomic so there is no chance of a crash or error causing unexpected behavior  
**access token** - send/recieve to browser/application

new list:

[ ] - route for (non-private) user info (/who_am_i) (username, highscore, most recent game, etc)
[ ] - route for checking if a username is a valid user or not

## How Routes should respond

**"error": value** - for retruning errors via json (should be handled by the handle_error() function)
**"response": value** - for returning a message in text (e.g. response: "user not found")
**"respnse/error": value, "data": value** - for returning data from the request

### Small notes

**cargo run** - runs the server **without** migrating the DB first
**cargo run migrate** - migrates the server then runs
**build documentation** cargo doc --no-deps
