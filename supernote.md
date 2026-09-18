Socket 1 Responds!
-Create function that opens connection, gets data out and closes it
-This data request should be async, and the struct that recieves the date
Ex: Window, should use Options for any data that is collected this way
Parsing & Testing
- Collect real examples of all events
- Use these to write parsing tests for each data type
- At runtime check for, reject, and log any malformed data or unfamiliar events/fields
(understands hyperland?)
- How to alert user if parser no longer

- real data will help verify generated data model
- Once parsing is working and data is converted to rust format, figure out how to use socket 1
- Write & test code to parse responses for requests I care about
- Update de rta modal to accept parsed socket l data
Do a pass for runtime checks & Logging
Output Logs to files
- By this point, f have events converting to rust data, and requests collecting suplementary dutea from sockel I
I should have considerew all available hypr land dat ce by ~~this point~~ model is settled
File watching, or (rsqlite?) good fork, /hypr monitor
Then Tui, want to see (live monitoring,) than playback
