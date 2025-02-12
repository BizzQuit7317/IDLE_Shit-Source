# IDLE_Shit-Source
The full documented source code for IDlE_Shit

# Game Settings
- The default screen size is 1280 x 720
- This is changeable currently only manually in config file

# Game Info
- The currency will be called Creature Points (cp) and will be measured in TeraBytes (Tb) *ifykyk*

# Storing Files
- Player data is stored as a binary file
- Creature data is stored in a current binary file
- Once creature dies that current binary gets stored under the creatures name as a binary
- Consumable items are stored as JSON files
- The config file is stored in a JSON format
- All assets saved as 24-bit Bitmap

# Adding feature
- When adding a new feature follow the steps below to add it properly
- Design layout of new page and know functionality
- In BasicApp structure add the layout
- Add all appropriate button, labels and anything else to the disable all function
- Create any neccecary function
- Any logic function should be called from a seperate file for readability
- Any page changing buttons should have logic in function

# Bugs
- If you rebirth then quite before saving it will corupt the save file <currently saving on exit_routine to ensure file is not corupted>
