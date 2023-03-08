
def test():
    data = requests.get('https://api.github.com').json()
    print(data)
    try: 
        return data["following_url"]
    except:
        return "error bro"
