---
order: 500
icon: log
---

# Search Playlists

Search playlists by query.

!!! Note
<https://jiosaavn.shuttleapp.rs> is only meant to demo the API and has rate-limiting enabled to minimise bandwidth consumption.
It is recommended to deploy your own instance for personal use.
!!!

+++ Request

HTTP

```bash
https://jiosaavn.shuttleapp.rs/search/playlists?query=nach+le
```

cURL

```bash
curl -X GET 'https://jiosaavn.shuttleapp.rs/search/playlists?query=nach+le' \
 -H 'content-type: application/json'
```

+++ Response

```json
{
	"status": "Success",
	"message": "✅ Search results for playlists fetched successfully!",
	"data": {
		"total": 3,
		"start": 1,
		"results": [
			{
				"id": "5148894",
				"name": "Nach Le",
				"subtitle": "Saavn",
				"type": "playlist",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/editorial/NachLe_20230809043316_50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/editorial/NachLe_20230809043316_150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/editorial/NachLe_20230809043316_500x500.jpg"
					}
				],
				"url": "https://www.jiosaavn.com/featured/nach-le/UQjTO6rFZfc_",
				"explicit": false,
				"description": null,
				"firstname": "Saavn",
				"lastname": "Editor",
				"language": "hindi",
				"user_id": "phulki_user",
				"entity_type": "playlist",
				"song_count": "24",
				"last_updated": null
			},
			{
				"id": "907322472",
				"name": "Aaja Nach Le - Weekly Jukebox",
				"subtitle": "Saavn",
				"type": "playlist",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/editorial/hindi-cluster_6238_20230807032324_50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/editorial/hindi-cluster_6238_20230807032324_150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/editorial/hindi-cluster_6238_20230807032324_500x500.jpg"
					}
				],
				"url": "https://www.jiosaavn.com/featured/aaja-nach-le-weekly-jukebox/QfSgtgl3xTjc1EngHtQQ2g__",
				"explicit": false,
				"description": null,
				"firstname": "Saavn",
				"lastname": "Editor",
				"language": "hindi",
				"user_id": "phulki_user",
				"entity_type": "playlist",
				"song_count": "25",
				"last_updated": null
			},
			{
				"id": "824478679",
				"name": "Pyaar Mein Nach Le - Weekly Jukebox",
				"subtitle": "Saavn",
				"type": "playlist",
				"image": [
					{
						"quality": "50x50",
						"link": "https://c.saavncdn.com/editorial/hindi-cluster_173_20230807030137_50x50.jpg"
					},
					{
						"quality": "150x150",
						"link": "https://c.saavncdn.com/editorial/hindi-cluster_173_20230807030137_150x150.jpg"
					},
					{
						"quality": "500x500",
						"link": "https://c.saavncdn.com/editorial/hindi-cluster_173_20230807030137_500x500.jpg"
					}
				],
				"url": "https://www.jiosaavn.com/featured/pyaar-mein-nach-le-weekly-jukebox/Kw51QoHqDfeO0eMLZZxqsA__",
				"explicit": false,
				"description": null,
				"firstname": "Saavn",
				"lastname": "Editor",
				"language": "hindi",
				"user_id": "phulki_user",
				"entity_type": "playlist",
				"song_count": "25",
				"last_updated": null
			}
		]
	}
}
```

+++

| Query Parameter                          | Description             | Default | Required                                |
| ---------------------------------------- | ----------------------- | ------- | --------------------------------------- |
| [!badge variant="contrast" text="query"] | Search query            | -       | [!badge variant="primary" text="True"]  |
| [!badge variant="contrast" text="page"]  | Current page            | 1       | [!badge variant="primary" text="False"] |
| [!badge variant="contrast" text="limit"] | limit number of results | 10      | [!badge variant="primary" text="False"] |
