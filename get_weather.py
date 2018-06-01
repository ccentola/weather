# load packages
import numpy as np
import pandas as pd
import requests
import datetime

# today
today = str(datetime.datetime.now().strftime("%Y%m%d"))

# parameters
key = 'fc197db221e7649af22e69473d62b681' # api key
url = 'http://api.openweathermap.org/data/2.5/weather' # url

# payload is dict of key and zipcode
payload = {
    'APPID':'fc197db221e7649af22e69473d62b681',
    'units':'imperial',
    'zip':'73301'
          }

# gather data for a list of zipcodes
cities = ['02115','73301','85054']
w = []
for c in cities:
    payload['zip'] = c
    #print(payload)
    r = requests.get(url,params=payload)
    w.append(r.json())

lat = []
lon = []
dt = []
loc_id = []
humidity = []
pressure = []
temp = []
temp_max = []
temp_min = []
name = []
wind_deg = []
wind_speed = []

for i in w:
    lat.append(i['coord']['lat'])
    lon.append(i['coord']['lon'])
    dt.append(i['dt'])
    loc_id.append(i['id'])
    humidity.append(i['main']['humidity'])
    pressure.append(i['main']['pressure'])
    temp.append(i['main']['temp'])
    temp_max.append(i['main']['temp_max'])
    temp_min.append(i['main']['temp_min'])
    name.append(i['name'])
    wind_deg.append(i['wind']['deg'])
    wind_speed.append(i['wind']['speed'])

# make location dataframe
location = pd.DataFrame(
    {
        'zip':cities,
        'lat':lat,
        'lon':lon,
        'id':loc_id,
        'name':name
    }
)

# make weather dataframe
weather = pd.DataFrame(
    {
        'humidity':humidity,
        'pressure':pressure,
        'temp':temp,
        'temp_max':temp_max,
        'temp_min':temp_min,
        'wind_deg':wind_deg,
        'wind_speed':wind_speed,
        'id':loc_id,
        'dt':dt,
    })

# make location csv
location.to_csv('/Users/centola/Dropbox/projects/weather/data/location_' + \
               today+'.csv', \
               columns = ['id','name','zip','lat','lon'],header=True, 
               index = False)

# make weather csv
weather.to_csv('/Users/centola/Dropbox/projects/weather/data/weather_' + \
               today+'.csv', \
               columns = ['id','dt','temp','temp_max',
                          'temp_min','pressure','humidity',
                          'wind_deg','wind_speed'],header=True, 
               index = False)