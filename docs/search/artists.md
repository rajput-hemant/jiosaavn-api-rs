---
order: 400
icon: person
---

# Search Artists

Search artists by query.

!!! Note
<https://jiosaavn.shuttleapp.rs> is only meant to demo the API and has rate-limiting enabled to minimise bandwidth consumption.
It is recommended to deploy your own instance for personal use.
!!!

+++ Request

HTTP

```bash
https://jiosaavn.shuttleapp.rs/search/artists?query=sia
```

cURL

```bash
curl -X GET 'https://jiosaavn.shuttleapp.rs/search/artists?query=sia' \
 -H 'content-type: application/json'
```

+++ Response

```json
{
  "status": "Success",
  "message": "✅ Search results for artists fetched successfully!",
  "data": {
    "total": 4521,
    "start": 1,
    "results": [
      {
        "id": "568707",
        "name": "Sia",
        "image": "https://c.saavncdn.com/artists/Sia_002_20200921152829_50x50.jpg",
        "extra": null,
        "role": "Artist",
        "type": "artist",
        "isRadioPresent": true,
        "entity": 1,
        "description": null,
        "position": null
      },
      {
        "id": "1988757",
        "name": "Sia Furler",
        "image": "https://www.jiosaavn.com/_i/3.0/artist-default-music.png",
        "extra": null,
        "role": "Artist",
        "type": "artist",
        "isRadioPresent": true,
        "entity": 1,
        "description": null,
        "position": null
      },
      {
        "id": "9210213",
        "name": "SIAHN",
        "image": "https://www.jiosaavn.com/_i/3.0/artist-default-film.png",
        "extra": null,
        "role": "Artist",
        "type": "artist",
        "isRadioPresent": false,
        "entity": 0,
        "description": null,
        "position": null
      },
      {
        "id": "13507479",
        "name": "Sia, R3HAB",
        "image": "https://www.jiosaavn.com/_i/3.0/artist-default-music.png",
        "extra": null,
        "role": "Artist",
        "type": "artist",
        "isRadioPresent": false,
        "entity": 0,
        "description": null,
        "position": null
      },
      {
        "id": "15100046",
        "name": "Sia, sped up + slowed",
        "image": "https://www.jiosaavn.com/_i/3.0/artist-default-music.png",
        "extra": null,
        "role": "Artist",
        "type": "artist",
        "isRadioPresent": false,
        "entity": 0,
        "description": null,
        "position": null
      },
      {
        "id": "4830659",
        "name": "Santanu Kumar Sia",
        "image": "https://www.jiosaavn.com/_i/3.0/artist-default-music.png",
        "extra": null,
        "role": "Artist",
        "type": "artist",
        "isRadioPresent": true,
        "entity": 0,
        "description": null,
        "position": null
      },
      {
        "id": "5917646",
        "name": "Lil Wayne, Sia, Diplo & Labrinth",
        "image": "https://www.jiosaavn.com/_i/3.0/artist-default-music.png",
        "extra": null,
        "role": "Artist",
        "type": "artist",
        "isRadioPresent": false,
        "entity": 0,
        "description": null,
        "position": null
      },
      {
        "id": "482728",
        "name": "Amit Sial",
        "image": "https://www.jiosaavn.com/_i/3.0/artist-default-film.png",
        "extra": null,
        "role": "Artist",
        "type": "artist",
        "isRadioPresent": true,
        "entity": 1,
        "description": null,
        "position": null
      },
      {
        "id": "8357078",
        "name": "Arin Siam",
        "image": "https://www.jiosaavn.com/_i/3.0/artist-default-film.png",
        "extra": null,
        "role": "Artist",
        "type": "artist",
        "isRadioPresent": false,
        "entity": 0,
        "description": null,
        "position": null
      },
      {
        "id": "9565511",
        "name": "Sian, Will Clarke, Amok (US)",
        "image": "https://www.jiosaavn.com/_i/3.0/artist-default-film.png",
        "extra": null,
        "role": "Artist",
        "type": "artist",
        "isRadioPresent": true,
        "entity": 0,
        "description": null,
        "position": null
      }
    ]
  }
}
```

+++

|           **Query Parameter**            |     **Description**     | **Default** |              **Required**               |
| :--------------------------------------: | :---------------------: | :---------: | :-------------------------------------: |
| [!badge variant="contrast" text="query"] |      Search query       |      -      | [!badge variant="primary" text="True"]  |
| [!badge variant="contrast" text="page"]  |      Current page       |      1      | [!badge variant="primary" text="False"] |
| [!badge variant="contrast" text="limit"] | limit number of results |     10      | [!badge variant="primary" text="False"] |
