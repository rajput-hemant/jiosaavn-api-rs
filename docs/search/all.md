---
order: 900
icon: globe
---

# Search All

Search songs, albums, playlists, artists, shows etc with a query.

!!! Note
<https://jiosaavn.shuttleapp.rs> is only meant to demo the API and has rate-limiting enabled to minimise bandwidth consumption.
It is recommended to deploy your own instance for personal use.
!!!

+++ Request

HTTP

```bash
https://jiosaavn.shuttleapp.rs/search/all?query=ram+siya+ram
```

cURL

```bash
curl -X GET 'https://jiosaavn.shuttleapp.rs/search/all?query=ram+siya+ram' \
 -H 'content-type: application/json'
```

+++ Response

```json
{
	"status": "Success",
	"message": "✅ Search results fetched successfully!",
	"data": {
		"topquery": {
			"position": 0,
			"results": [
				{
					"id": "HLulXlir",
					"name": "Ram Siya Ram",
					"subtitle": "Ram Siya Ram",
					"type": "song",
					"image": [
						{
							"quality": "50x50",
							"link": "https://c.saavncdn.com/215/Ram-Siya-Ram-Hindi-2021-20210222031001-50x50.jpg"
						},
						{
							"quality": "150x150",
							"link": "https://c.saavncdn.com/215/Ram-Siya-Ram-Hindi-2021-20210222031001-150x150.jpg"
						},
						{
							"quality": "500x500",
							"link": "https://c.saavncdn.com/215/Ram-Siya-Ram-Hindi-2021-20210222031001-500x500.jpg"
						}
					],
					"url": "https://www.jiosaavn.com/song/ram-siya-ram/OCQeXSxcXkE",
					"explicit": false,
					"description": "Song by Sachet Tandon",
					"music": null,
					"year": null,
					"isMovie": null,
					"language": "hindi",
					"songIds": null,
					"score": "128097128",
					"primaryArtists": "Sachet Tandon",
					"singers": "Sachet Tandon"
				}
			]
		},
		"albums": {
			"position": 1,
			"results": [
				{
					"id": "25500145",
					"name": "Ram Siya Ram",
					"subtitle": "",
					"type": "album",
					"image": [
						{
							"quality": "50x50",
							"link": "https://c.saavncdn.com/215/Ram-Siya-Ram-Hindi-2021-20210222031001-50x50.jpg"
						},
						{
							"quality": "150x150",
							"link": "https://c.saavncdn.com/215/Ram-Siya-Ram-Hindi-2021-20210222031001-150x150.jpg"
						},
						{
							"quality": "500x500",
							"link": "https://c.saavncdn.com/215/Ram-Siya-Ram-Hindi-2021-20210222031001-500x500.jpg"
						}
					],
					"url": "https://www.jiosaavn.com/album/ram-siya-ram/cy9LsEw-pn0_",
					"explicit": false,
					"description": "2021 · Hindi Album · Sachet Tandon",
					"music": "Sachet Tandon",
					"year": 2021,
					"isMovie": false,
					"language": "hindi",
					"songIds": "HLulXlir"
				},
				{
					"id": "24471788",
					"name": "Ram Siya Ram",
					"subtitle": "",
					"type": "album",
					"image": [
						{
							"quality": "50x50",
							"link": "https://c.saavncdn.com/975/Ram-Siya-Ram-Hindi-2021-20210106012535-50x50.jpg"
						},
						{
							"quality": "150x150",
							"link": "https://c.saavncdn.com/975/Ram-Siya-Ram-Hindi-2021-20210106012535-150x150.jpg"
						},
						{
							"quality": "500x500",
							"link": "https://c.saavncdn.com/975/Ram-Siya-Ram-Hindi-2021-20210106012535-500x500.jpg"
						}
					],
					"url": "https://www.jiosaavn.com/album/ram-siya-ram/gz,aRUzsOs0_",
					"explicit": false,
					"description": "2021 · Hindi Album · Rakesh Kala",
					"music": "Rakesh Kala",
					"year": 2021,
					"isMovie": false,
					"language": "hindi",
					"songIds": "FNk9HwWp"
				},
				{
					"id": "41250229",
					"name": "Ram Siya Ram (Slowed &amp; Reverbed)",
					"subtitle": "",
					"type": "album",
					"image": [
						{
							"quality": "50x50",
							"link": "https://c.saavncdn.com/474/Ram-Siya-Ram-Slowed-Reverbed-English-2022-20221230230233-50x50.jpg"
						},
						{
							"quality": "150x150",
							"link": "https://c.saavncdn.com/474/Ram-Siya-Ram-Slowed-Reverbed-English-2022-20221230230233-150x150.jpg"
						},
						{
							"quality": "500x500",
							"link": "https://c.saavncdn.com/474/Ram-Siya-Ram-Slowed-Reverbed-English-2022-20221230230233-500x500.jpg"
						}
					],
					"url": "https://www.jiosaavn.com/album/ram-siya-ram-slowed-reverbed/vLMTlP1cDP0_",
					"explicit": false,
					"description": "2022 · Sanskrit Album · Dalip Shekhawat",
					"music": "Dalip Shekhawat",
					"year": 2022,
					"isMovie": false,
					"language": "sanskrit",
					"songIds": "DQtrXQP0"
				}
			]
		},
		"songs": {
			"position": 2,
			"results": [
				{
					"id": "HLulXlir",
					"name": "Ram Siya Ram",
					"subtitle": "Ram Siya Ram",
					"type": "song",
					"image": [
						{
							"quality": "50x50",
							"link": "https://c.saavncdn.com/215/Ram-Siya-Ram-Hindi-2021-20210222031001-50x50.jpg"
						},
						{
							"quality": "150x150",
							"link": "https://c.saavncdn.com/215/Ram-Siya-Ram-Hindi-2021-20210222031001-150x150.jpg"
						},
						{
							"quality": "500x500",
							"link": "https://c.saavncdn.com/215/Ram-Siya-Ram-Hindi-2021-20210222031001-500x500.jpg"
						}
					],
					"url": "https://www.jiosaavn.com/song/ram-siya-ram/OCQeXSxcXkE",
					"explicit": false,
					"description": "Ram Siya Ram · Sachet Tandon",
					"score": 128097128.0,
					"primaryArtists": "Sachet Tandon",
					"singers": "Sachet Tandon",
					"language": "hindi"
				},
				{
					"id": "dBM1AsPC",
					"name": "Ram Siya Ram",
					"subtitle": "Adipurush",
					"type": "song",
					"image": [
						{
							"quality": "50x50",
							"link": "https://c.saavncdn.com/972/Adipurush-Hindi-2023-20230607184755-50x50.jpg"
						},
						{
							"quality": "150x150",
							"link": "https://c.saavncdn.com/972/Adipurush-Hindi-2023-20230607184755-150x150.jpg"
						},
						{
							"quality": "500x500",
							"link": "https://c.saavncdn.com/972/Adipurush-Hindi-2023-20230607184755-500x500.jpg"
						}
					],
					"url": "https://www.jiosaavn.com/song/ram-siya-ram/FComADVDZ3A",
					"explicit": false,
					"description": "Adipurush · Sachet-Parampara, Sachet Tandon, Parampara Tandon",
					"score": 38266412.0,
					"primaryArtists": "Sachet-Parampara, Sachet Tandon, Parampara Tandon",
					"singers": "Sachet-Parampara, Sachet Tandon, Parampara Tandon",
					"language": "hindi"
				},
				{
					"id": "FNk9HwWp",
					"name": "Ram Siya Ram",
					"subtitle": "Ram Siya Ram",
					"type": "song",
					"image": [
						{
							"quality": "50x50",
							"link": "https://c.saavncdn.com/975/Ram-Siya-Ram-Hindi-2021-20210106012535-50x50.jpg"
						},
						{
							"quality": "150x150",
							"link": "https://c.saavncdn.com/975/Ram-Siya-Ram-Hindi-2021-20210106012535-150x150.jpg"
						},
						{
							"quality": "500x500",
							"link": "https://c.saavncdn.com/975/Ram-Siya-Ram-Hindi-2021-20210106012535-500x500.jpg"
						}
					],
					"url": "https://www.jiosaavn.com/song/ram-siya-ram/NiYACDxHYEM",
					"explicit": false,
					"description": "Ram Siya Ram · Rakesh Kala",
					"score": 10897612.0,
					"primaryArtists": "Rakesh Kala",
					"singers": "",
					"language": "hindi"
				}
			]
		},
		"playlists": {
			"position": 3,
			"results": []
		},
		"artists": {
			"position": 4,
			"results": [
				{
					"id": "15182145",
					"name": "Siya Ram Gurjar",
					"image": "https://www.jiosaavn.com/_i/3.0/artist-default-film.png",
					"extra": "primary",
					"role": null,
					"type": "artist",
					"isRadioPresent": false,
					"entity": 0,
					"description": "Primary",
					"position": 1
				},
				{
					"id": "670533",
					"name": "Siyaram Nishad",
					"image": "https://www.jiosaavn.com/_i/3.0/artist-default-music.png",
					"extra": "lyricist",
					"role": null,
					"type": "artist",
					"isRadioPresent": true,
					"entity": 0,
					"description": "Lyricist",
					"position": 2
				},
				{
					"id": "12289582",
					"name": "siyaram gurjar",
					"image": "https://www.jiosaavn.com/_i/3.0/artist-default-music.png",
					"extra": "music director",
					"role": null,
					"type": "artist",
					"isRadioPresent": true,
					"entity": 0,
					"description": "Music director",
					"position": 3
				}
			]
		},
		"shows": {
			"position": 5,
			"results": [
				{
					"id": "74222",
					"name": "Om Sai Ram Chants",
					"subtitle": "",
					"type": "show",
					"image": [
						{
							"quality": "50x50",
							"link": "https://c.sop.saavncdn.com/Om-Sai-Ram-Chants-20210623062328-500x500.jpg"
						},
						{
							"quality": "150x150",
							"link": "https://c.sop.saavncdn.com/Om-Sai-Ram-Chants-20210623062328-500x500.jpg"
						},
						{
							"quality": "500x500",
							"link": "https://c.sop.saavncdn.com/Om-Sai-Ram-Chants-20210623062328-500x500.jpg"
						}
					],
					"url": "https://www.jiosaavn.com/shows/om-sai-ram-chants/1/c73sacC8B6A_",
					"explicit": false,
					"description": "Season 1",
					"seasonNumber": 1
				},
				{
					"id": "15223",
					"name": "RAM Eighties Hit Radio Retro Mix 80s retro mix May 2019 by t.o.g",
					"subtitle": "",
					"type": "show",
					"image": [
						{
							"quality": "50x50",
							"link": "https://c.sop.saavncdn.com/RAM-FM-retro-mix-20200521113419-500x500.jpg"
						},
						{
							"quality": "150x150",
							"link": "https://c.sop.saavncdn.com/RAM-FM-retro-mix-20200521113419-500x500.jpg"
						},
						{
							"quality": "500x500",
							"link": "https://c.sop.saavncdn.com/RAM-FM-retro-mix-20200521113419-500x500.jpg"
						}
					],
					"url": "https://www.jiosaavn.com/shows/ram-eighties-hit-radio-retro-mix-80s-retro-mix-may-2019-by-t.o.g/1/bBFIfRXlS6o_",
					"explicit": false,
					"description": "Season 1",
					"seasonNumber": 1
				},
				{
					"id": "15947",
					"name": "RAM Eighties Hit Radio Retro 80s Mix Aug",
					"subtitle": "",
					"type": "show",
					"image": [
						{
							"quality": "50x50",
							"link": "https://c.sop.saavncdn.com/RAM-Eighties-Hit-Radio-Retro-80s-Mix-Aug-20200525192651-500x500.jpg"
						},
						{
							"quality": "150x150",
							"link": "https://c.sop.saavncdn.com/RAM-Eighties-Hit-Radio-Retro-80s-Mix-Aug-20200525192651-500x500.jpg"
						},
						{
							"quality": "500x500",
							"link": "https://c.sop.saavncdn.com/RAM-Eighties-Hit-Radio-Retro-80s-Mix-Aug-20200525192651-500x500.jpg"
						}
					],
					"url": "https://www.jiosaavn.com/shows/ram-eighties-hit-radio-retro-80s-mix-aug/1/8Uq4p0eSut0_",
					"explicit": false,
					"description": "Season 1",
					"seasonNumber": 1
				}
			]
		}
	}
}
```

+++

| **Query Paramter**                       | **Description** | **Required**                           |
| ---------------------------------------- | --------------- | -------------------------------------- |
| [!badge variant="contrast" text="query"] | Search query    | [!badge variant="primary" text="True"] |
