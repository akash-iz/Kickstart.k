@ask:location_to_install please provide the location where you want to check out the project
@log git project is going to checkout in location ${location_to_install}
@run:${location_to_install} git clone https://github.com/vitalecare/ve-frontend.git
//@log now trying npm install
//@run:${location_to_install}/ve-frontend/ve-frontend npm.cmd install
@log opening the project in vs code
@run:${location_to_install}/ve-frontend/ve-frontend code.cmd .