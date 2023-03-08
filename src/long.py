import requests
def test():
    data = requests.get('https://api.github.com').json()
    try: 
        return data["following_url"]
    except:
        return data
