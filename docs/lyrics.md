---
order: 1
icon: note
---

# Lyrics

!!! Note
<https://jiosaavn.shuttleapp.rs> is only meant to demo the API and has rate-limiting enabled to minimise bandwidth consumption.
It is recommended to deploy your own instance for personal use.
!!!

## Song lyrics by song ID

!!! Note
Lyrics are only available for a limited set of songs.
!!!

+++ Request

HTTP

```bash
https://jiosaavn.shuttleapp.rs/lyrics?id=mPTrDSun
```

cURL

```bash
curl -X GET 'https://jiosaavn.shuttleapp.rs/lyrics?id=mPTrDSun' \
 -H 'content-type: application/json'
```

+++ Response

```json
{
  "status": "Success",
  "message": "✅ Lyrics fetched successfully!",
  "data": {
    "lyrics": "Kaushlya Dasrath Ke Nandan<br>Ram Lalaat Pe Shobhit Chandan<br>Raghupati Ki Jai Bole Lakshman<br>Ram Siya Ka Ho Abhinandan<br><br>Anjan Putr Padhe Hain Charan Mein<br>Ram Siya Japte Tan Man Mein<br><br>Mangal Bhavan Amangal Haari<br>Drabahu Sudasarath Achar Bihari<br><br>Ram Siya Ram Siya Ram<br>Jai Jai Ram<br>Ram Siya Ram Siya Ram<br>Jai Jai Ram<br>Ram Siya Ram Siya Ram<br>Jai Jai Ram<br>Ram Siya Ram Siya Ram<br>Jai Jai Ram<br><br>Mere Tan Man Dhadkan Mein<br>Siya Ram Ram Hai<br>Man Mandir Ke Darpan Mein<br>Siya Ram Ram Hai<br><br>Tu Hi Siya Ka Ram<br>Radha Ka Tu Hi Shyam<br>Janmo Janam Ka Hi Ye Saath Hai<br><br>Mira Ka Tu Bhajan<br>Bhajate Hari Pavan<br>Tulsi Mein Bhi Likhi Ye Baat Hai<br><br>Mangal Bhavan Amangal Haari<br>Drabahu Sudasarath Achar Bihari<br><br>Ram Siya Ram Siya Ram<br>Jai Jai Ram<br>Ram Siya Ram Siya Ram<br>Jai Jai Ram<br>Ram Siya Ram Siya Ram<br>Jai Jai Ram<br>Ram Siya Ram Siya Ram<br>Jai Jai Ram<br><br>Mangal Bhavan Amangal Haari<br>Mangal Bhavan Amangal Haari<br>Drabahu Sudasarath Achar Bihari<br><br>Ram Siya Ram Siya Ram<br>Jai Jai Ram<br>Ram Siya Ram Siya Ram<br>Jai Jai Ram<br>Ram Siya Ram Siya Ram<br>Jai Jai Ram<br>Ram Siya Ram Siya Ram<br>Jai Jai Ram<br><br><br>",
    "lyricsCopyright": "Lyrics powered by JioSaavn",
    "snippet": "Lyrics"
  }
}
```

+++

|          **Query Parameter**          | **Description** |              **Required**              |
| :-----------------------------------: | :-------------: | :------------------------------------: |
| [!badge variant="contrast" text="id"] |     Song ID     | [!badge variant="primary" text="True"] |
