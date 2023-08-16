---
order: 1000
icon: package
---

# Homepage Data

Get launch data from <https://jiosaavn.com> homepage for different modules such as `songs`, `albums`, `trending`, `charts`, `playlists`, `artists`, `new-releases`, `podcasts`, `radio` and `featured-playlists`, etc.

!!! Note
<https://jiosaavn.shuttleapp.rs> is only meant to demo the API and has rate-limiting enabled to minimise bandwidth consumption.
It is recommended to deploy your own instance for personal use.
!!!

+++ Request

HTTP

```sh
https://jiosaavn.shuttleapp.rs/modules?language=hindi,english
```

cURL

```bash
curl -X GET 'https://jiosaavn.shuttleapp.rs/modules?language=hindi,english' \
 -H 'content-type: application/json'
```

+++ Response

```json
{
  "status": "Success",
  "message": "✅ Successfully fetched home data",
  "data": {
    "artistRecos": {
      "title": "Recommended Artist Stations",
      "subtitle": "",
      "featuredText": null,
      "data": [
        {
          "id": "459320",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/artists/Arijit_Singh_002_20230323062147_500x500.jpg"
            }
          ],
          "name": "Arijit Singh",
          "query": "Arijit Singh",
          "type": "radio_station",
          "url": "https://www.jiosaavn.com/artist/arijit-singh-songs/LlRWpHzy3Hk_",
          "subtitle": "Artist Radio",
          "explicit": false,
          "featuredStationType": "artist",
          "stationDisplayText": "Arijit Singh"
        },
        {
          "id": "3319750",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/artists/Sidhu_Moose_Wala_20190627113332_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/artists/Sidhu_Moose_Wala_20190627113332_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/artists/Sidhu_Moose_Wala_20190627113332_500x500.jpg"
            }
          ],
          "name": "Sidhu Moose Wala",
          "query": "Sidhu Moose Wala",
          "type": "radio_station",
          "url": "https://www.jiosaavn.com/artist/sidhu-moose-wala-songs/ylevcL-ZuH8_",
          "subtitle": "Artist Radio",
          "explicit": false,
          "featuredStationType": "artist",
          "stationDisplayText": "Sidhu Moose Wala"
        },
        {
          "id": "788130",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/artists/B_Praak_001_20191118112005_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/artists/B_Praak_001_20191118112005_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/artists/B_Praak_001_20191118112005_500x500.jpg"
            }
          ],
          "name": "B Praak",
          "query": "B Praak",
          "type": "radio_station",
          "url": "https://www.jiosaavn.com/artist/b-praak-songs/CfABr-vmQdw_",
          "subtitle": "Artist Radio",
          "explicit": false,
          "featuredStationType": "artist",
          "stationDisplayText": "B Praak"
        },
        {
          "id": "3436900",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/artists/Hansraj_Raghuwanshi_001_20220916054832_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/artists/Hansraj_Raghuwanshi_001_20220916054832_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/artists/Hansraj_Raghuwanshi_001_20220916054832_500x500.jpg"
            }
          ],
          "name": "Hansraj Raghuwanshi",
          "query": "Hansraj Raghuwanshi",
          "type": "radio_station",
          "url": "https://www.jiosaavn.com/artist/hansraj-raghuwanshi-songs/sF6m,UAR8co_",
          "subtitle": "Artist Radio",
          "explicit": false,
          "featuredStationType": "artist",
          "stationDisplayText": "Hansraj Raghuwanshi"
        },
        {
          "id": "881158",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/artists/Jubin_Nautiyal_002_20180507091834_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/artists/Jubin_Nautiyal_002_20180507091834_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/artists/Jubin_Nautiyal_002_20180507091834_500x500.jpg"
            }
          ],
          "name": "Jubin Nautiyal",
          "query": "Jubin Nautiyal",
          "type": "radio_station",
          "url": "https://www.jiosaavn.com/artist/jubin-nautiyal-songs/uGdfg6zGf4s_",
          "subtitle": "Artist Radio",
          "explicit": false,
          "featuredStationType": "artist",
          "stationDisplayText": "Jubin Nautiyal"
        },
        {
          "id": "455127",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/artists/Udit_Narayan_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/artists/Udit_Narayan_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/artists/Udit_Narayan_500x500.jpg"
            }
          ],
          "name": "Udit Narayan",
          "query": "Udit Narayan",
          "type": "radio_station",
          "url": "https://www.jiosaavn.com/artist/udit-narayan-songs/kLtmb7Vh8Rs_",
          "subtitle": "Artist Radio",
          "explicit": false,
          "featuredStationType": "artist",
          "stationDisplayText": "Udit Narayan"
        },
        {
          "id": "455120",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/artists/Alka_Yagnik_002_20220314192930_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/artists/Alka_Yagnik_002_20220314192930_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/artists/Alka_Yagnik_002_20220314192930_500x500.jpg"
            }
          ],
          "name": "Alka Yagnik",
          "query": "Alka Yagnik",
          "type": "radio_station",
          "url": "https://www.jiosaavn.com/artist/alka-yagnik-songs/uqRkqsl4ZnQ_",
          "subtitle": "Artist Radio",
          "explicit": false,
          "featuredStationType": "artist",
          "stationDisplayText": "Alka Yagnik"
        },
        {
          "id": "4698047",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/artists/Jass_Manak_20190215063438_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/artists/Jass_Manak_20190215063438_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/artists/Jass_Manak_20190215063438_500x500.jpg"
            }
          ],
          "name": "Jass Manak",
          "query": "Jass Manak",
          "type": "radio_station",
          "url": "https://www.jiosaavn.com/artist/jass-manak-songs/uf42s4,N9LY_",
          "subtitle": "Artist Radio",
          "explicit": false,
          "featuredStationType": "artist",
          "stationDisplayText": "Jass Manak"
        },
        {
          "id": "464656",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/artists/Armaan_Malik_003_20220107135753_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/artists/Armaan_Malik_003_20220107135753_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/artists/Armaan_Malik_003_20220107135753_500x500.jpg"
            }
          ],
          "name": "Armaan Malik",
          "query": "Armaan Malik",
          "type": "radio_station",
          "url": "https://www.jiosaavn.com/artist/armaan-malik-songs/1iZ7Gi0bi1Y_",
          "subtitle": "Artist Radio",
          "explicit": false,
          "featuredStationType": "artist",
          "stationDisplayText": "Armaan Malik"
        },
        {
          "id": "485956",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/artists/Yo_Yo_Honey_Singh_002_20221216102650_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/artists/Yo_Yo_Honey_Singh_002_20221216102650_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/artists/Yo_Yo_Honey_Singh_002_20221216102650_500x500.jpg"
            }
          ],
          "name": "Yo Yo Honey Singh",
          "query": "Yo Yo Honey Singh",
          "type": "radio_station",
          "url": "https://www.jiosaavn.com/artist/yo-yo-honey-singh-songs/06QxyAvVpB4_",
          "subtitle": "Artist Radio",
          "explicit": false,
          "featuredStationType": "artist",
          "stationDisplayText": "Yo Yo Honey Singh"
        },
        {
          "id": "455109",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/artists/Lata_Mangeshkar_004_20230623105323_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/artists/Lata_Mangeshkar_004_20230623105323_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/artists/Lata_Mangeshkar_004_20230623105323_500x500.jpg"
            }
          ],
          "name": "Lata Mangeshkar",
          "query": "Lata Mangeshkar",
          "type": "radio_station",
          "url": "https://www.jiosaavn.com/artist/lata-mangeshkar-songs/FCtl69DObYg_",
          "subtitle": "Artist Radio",
          "explicit": false,
          "featuredStationType": "artist",
          "stationDisplayText": "Lata Mangeshkar"
        },
        {
          "id": "464932",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/artists/Neha_Kakkar_006_20200822042626_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/artists/Neha_Kakkar_006_20200822042626_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/artists/Neha_Kakkar_006_20200822042626_500x500.jpg"
            }
          ],
          "name": "Neha Kakkar",
          "query": "Neha Kakkar",
          "type": "radio_station",
          "url": "https://www.jiosaavn.com/artist/neha-kakkar-songs/EkEBV7JAU-I_",
          "subtitle": "Artist Radio",
          "explicit": false,
          "featuredStationType": "artist",
          "stationDisplayText": "Neha Kakkar"
        },
        {
          "id": "455144",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/artists/Kishore_Kumar_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/artists/Kishore_Kumar_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/artists/Kishore_Kumar_500x500.jpg"
            }
          ],
          "name": "Kishore Kumar",
          "query": "Kishore Kumar",
          "type": "radio_station",
          "url": "https://www.jiosaavn.com/artist/kishore-kumar-songs/yMoPyl3ZzyY_",
          "subtitle": "Artist Radio",
          "explicit": false,
          "featuredStationType": "artist",
          "stationDisplayText": "Kishore Kumar"
        },
        {
          "id": "505312",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/artists/Mohammed_Rafi_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/artists/Mohammed_Rafi_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/artists/Mohammed_Rafi_500x500.jpg"
            }
          ],
          "name": "Mohammed Rafi",
          "query": "Mohammed Rafi",
          "type": "radio_station",
          "url": "https://www.jiosaavn.com/artist/mohammed-rafi-songs/UY8fLYUk,Uo_",
          "subtitle": "Artist Radio",
          "explicit": false,
          "featuredStationType": "artist",
          "stationDisplayText": "Mohammed Rafi"
        },
        {
          "id": "457146",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/artists/Rahat_Fateh_Ali_Khan_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/artists/Rahat_Fateh_Ali_Khan_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/artists/Rahat_Fateh_Ali_Khan_500x500.jpg"
            }
          ],
          "name": "Rahat Fateh Ali Khan",
          "query": "Rahat Fateh Ali Khan",
          "type": "radio_station",
          "url": "https://www.jiosaavn.com/artist/rahat-fateh-ali-khan-songs/EesgMPGWL90_",
          "subtitle": "Artist Radio",
          "explicit": false,
          "featuredStationType": "artist",
          "stationDisplayText": "Rahat Fateh Ali Khan"
        },
        {
          "id": "12040540",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/artists/Kaka_000_20211215124002_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/artists/Kaka_000_20211215124002_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/artists/Kaka_000_20211215124002_500x500.jpg"
            }
          ],
          "name": "Kaka",
          "query": "Kaka",
          "type": "radio_station",
          "url": "https://www.jiosaavn.com/artist/kaka-songs/HdzwVwZ5Lf8_",
          "subtitle": "Artist Radio",
          "explicit": false,
          "featuredStationType": "artist",
          "stationDisplayText": "Kaka"
        },
        {
          "id": "459633",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/artists/Atif_Aslam_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/artists/Atif_Aslam_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/artists/Atif_Aslam_500x500.jpg"
            }
          ],
          "name": "Atif Aslam",
          "query": "Atif Aslam",
          "type": "radio_station",
          "url": "https://www.jiosaavn.com/artist/atif-aslam-songs/MXn8bhT308U_",
          "subtitle": "Artist Radio",
          "explicit": false,
          "featuredStationType": "artist",
          "stationDisplayText": "Atif Aslam"
        },
        {
          "id": "456863",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/artists/Badshah_005_20230608084021_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/artists/Badshah_005_20230608084021_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/artists/Badshah_005_20230608084021_500x500.jpg"
            }
          ],
          "name": "Badshah",
          "query": "Badshah",
          "type": "radio_station",
          "url": "https://www.jiosaavn.com/artist/badshah-songs/d4OwAaEcnD0_",
          "subtitle": "Artist Radio",
          "explicit": false,
          "featuredStationType": "artist",
          "stationDisplayText": "Badshah"
        },
        {
          "id": "455782",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/artists/KK_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/artists/KK_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/artists/KK_500x500.jpg"
            }
          ],
          "name": "KK",
          "query": "KK",
          "type": "radio_station",
          "url": "https://www.jiosaavn.com/artist/kk-songs/K,5uNVM,qpM_",
          "subtitle": "Artist Radio",
          "explicit": false,
          "featuredStationType": "artist",
          "stationDisplayText": "KK"
        },
        {
          "id": "5007756",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/artists/Sapna_Choudhary_000_20220916102820_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/artists/Sapna_Choudhary_000_20220916102820_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/artists/Sapna_Choudhary_000_20220916102820_500x500.jpg"
            }
          ],
          "name": "Sapna Choudhary",
          "query": "Sapna Choudhary",
          "type": "radio_station",
          "url": "https://www.jiosaavn.com/artist/sapna-choudhary-songs/VHv7Fm7wcX4_",
          "subtitle": "Artist Radio",
          "explicit": false,
          "featuredStationType": "artist",
          "stationDisplayText": "Sapna Choudhary"
        }
      ]
    },
    "discover": [
      {
        "id": "122",
        "name": "Music Is Life",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/Mb5vxykOf7-station__20230607053819.jpg",
        "url": "https://www.jiosaavn.com/s/channel/music-is-life/rJjhdKvoMdY_",
        "badge": "",
        "subType": "genre",
        "isFeatured": false,
        "videoUrl": "",
        "videoThumbnail": "",
        "explicit": false
      },
      {
        "id": "27",
        "name": "Romance",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/RPQXRCUT55-station__20221007155650.jpg",
        "url": "https://www.jiosaavn.com/s/channel/romance/SqI6f167Uoo_",
        "badge": "",
        "subType": "mood",
        "isFeatured": false,
        "videoUrl": "http://c.saavncdn.com/editorial/videos/romance_3.mp4",
        "videoThumbnail": "https://c.saavncdn.com/editorial/videos/romance_2.jpg",
        "explicit": false
      },
      {
        "id": "189",
        "name": "The College Playlists",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/i39f98BI9I-CollegePlaylists_20230515083010.jpg",
        "url": "https://www.jiosaavn.com/s/channel/the-college-playlists/fnBZ8dFeIoM_",
        "badge": "",
        "subType": "mood",
        "isFeatured": false,
        "videoUrl": "",
        "videoThumbnail": "",
        "explicit": false
      },
      {
        "id": "40",
        "name": "Dance",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/R627Kir5MG-station__20221007160900.jpg",
        "url": "https://www.jiosaavn.com/s/channel/dance/,06BpBddRTw_",
        "badge": "",
        "subType": "mood",
        "isFeatured": false,
        "videoUrl": "http://c.saavncdn.com/editorial/videos/dance_3.mp4",
        "videoThumbnail": "https://c.saavncdn.com/editorial/videos/dance_2.jpg",
        "explicit": false
      },
      {
        "id": "29",
        "name": "Workout",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/MPFnHRHXoL-station__20220317063811.jpg",
        "url": "https://www.jiosaavn.com/s/channel/workout/hR6iDdclo1Y_",
        "badge": "",
        "subType": "mood",
        "isFeatured": false,
        "videoUrl": "http://c.saavncdn.com/editorial/videos/workout_3.mp4",
        "videoThumbnail": "https://c.saavncdn.com/editorial/videos/workout_2.jpg",
        "explicit": false
      },
      {
        "id": "46",
        "name": "The 1990s",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/pFq3HgYvgq-station__20220317063205.jpg",
        "url": "https://www.jiosaavn.com/s/channel/the-1990s/USAZn3XRH3w_",
        "badge": "",
        "subType": "mood",
        "isFeatured": false,
        "videoUrl": "http://c.saavncdn.com/editorial/videos/1990_2.mp4",
        "videoThumbnail": "https://c.saavncdn.com/editorial/videos/the-1990s_2.jpg",
        "explicit": false
      },
      {
        "id": "76",
        "name": "Happy",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/0k7X2hojVl-station__20220317070807.jpg",
        "url": "https://www.jiosaavn.com/s/channel/happy/,wK4QKrlCGQ_",
        "badge": "",
        "subType": "mood",
        "isFeatured": false,
        "videoUrl": "http://c.saavncdn.com/editorial/videos/happy_3.mp4",
        "videoThumbnail": "https://c.saavncdn.com/editorial/videos/happy_2.jpg",
        "explicit": false
      },
      {
        "id": "187",
        "name": "Best Of 2022",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/A2rvRbZ9DY-station__20221224094334.jpg",
        "url": "https://www.jiosaavn.com/s/channel/best-of-2022/9oShOsdgZtw_",
        "badge": "",
        "subType": "genre",
        "isFeatured": false,
        "videoUrl": "",
        "videoThumbnail": "",
        "explicit": false
      },
      {
        "id": "16",
        "name": "Chill",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/oVJC5O186o-station__20220317062833.jpg",
        "url": "https://www.jiosaavn.com/s/channel/chill/3TJKKmQoIDo_",
        "badge": "",
        "subType": "mood",
        "isFeatured": false,
        "videoUrl": "http://c.saavncdn.com/editorial/videos/chill_3.mp4",
        "videoThumbnail": "https://c.saavncdn.com/editorial/videos/chill_2.jpg",
        "explicit": false
      },
      {
        "id": "17",
        "name": "Party",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/Y4sq41wsOV-station__20220317071322.jpg",
        "url": "https://www.jiosaavn.com/s/channel/party/p9xkUPVqX6o_",
        "badge": "",
        "subType": "genre",
        "isFeatured": false,
        "videoUrl": "http://c.saavncdn.com/editorial/videos/party_3.mp4",
        "videoThumbnail": "https://c.saavncdn.com/editorial/videos/party_2.jpg",
        "explicit": false
      },
      {
        "id": "164",
        "name": "Best Of 2020",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/zgh9U1OYFV-station__20220317125748.jpg",
        "url": "https://www.jiosaavn.com/s/channel/best-of-2020/069z8RjU5wA_",
        "badge": "",
        "subType": "genre",
        "isFeatured": false,
        "videoUrl": "",
        "videoThumbnail": "",
        "explicit": false
      },
      {
        "id": "104",
        "name": "Top JioTunes",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/21Q9prRxIb-station__20220317064402.jpg",
        "url": "https://www.jiosaavn.com/s/channel/top-jiotunes/7v3-B4T-XII_",
        "badge": "",
        "subType": "genre",
        "isFeatured": false,
        "videoUrl": "http://c.saavncdn.com/editorial/videos/JioTunes_2.mp4",
        "videoThumbnail": "https://c.saavncdn.com/editorial/videos/top-jiotunes_2.jpg",
        "explicit": false
      },
      {
        "id": "134",
        "name": "The 2010s",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/gDUTBFQugQ-station__20220317082745.jpg",
        "url": "https://www.jiosaavn.com/s/channel/the-2010s/5nJfByulXgU_",
        "badge": "",
        "subType": "genre",
        "isFeatured": false,
        "videoUrl": "http://c.saavncdn.com/editorial/videos/2010_2.mp4",
        "videoThumbnail": "https://c.saavncdn.com/editorial/videos/the-2010s_2.jpg",
        "explicit": false
      },
      {
        "id": "180",
        "name": "Best Of 2021",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/3qRrcPALVL-station__20220317125427.jpg",
        "url": "https://www.jiosaavn.com/s/channel/best-of-2021/bYdEDIHS3lc_",
        "badge": "",
        "subType": "genre",
        "isFeatured": false,
        "videoUrl": "",
        "videoThumbnail": "",
        "explicit": false
      },
      {
        "id": "143",
        "name": "The 2000s",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/yatuQSdoLf-station__20201020101716.jpg",
        "url": "https://www.jiosaavn.com/s/channel/the-2000s/v61mHqVCotI_",
        "badge": "",
        "subType": "mood",
        "isFeatured": false,
        "videoUrl": "",
        "videoThumbnail": "",
        "explicit": false
      },
      {
        "id": "37",
        "name": "Hip Hop",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/SBnwFu8m4F-station__20221007161600.jpg",
        "url": "https://www.jiosaavn.com/s/channel/hip-hop/89HV0q4izuM_",
        "badge": "",
        "subType": "genre",
        "isFeatured": false,
        "videoUrl": "http://c.saavncdn.com/editorial/videos/hip-hop_3.mp4",
        "videoThumbnail": "https://c.saavncdn.com/editorial/videos/hip-hop_2.jpg",
        "explicit": false
      },
      {
        "id": "38",
        "name": "EDM",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/xIdlpa4YTO-station__20221007160803.jpg",
        "url": "https://www.jiosaavn.com/s/channel/edm/o-exXhXX2-M_",
        "badge": "",
        "subType": "genre",
        "isFeatured": false,
        "videoUrl": "http://c.saavncdn.com/editorial/videos/EDM_2.mp4",
        "videoThumbnail": "https://c.saavncdn.com/editorial/videos/edm_2.jpg",
        "explicit": false
      },
      {
        "id": "30",
        "name": "Devotional",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/XN2HDFIBtp-station__20220317074753.jpg",
        "url": "https://www.jiosaavn.com/s/channel/devotional/8l,H48KVrl8_",
        "badge": "",
        "subType": "genre",
        "isFeatured": false,
        "videoUrl": "http://c.saavncdn.com/editorial/videos/devotional_3.mp4",
        "videoThumbnail": "https://c.saavncdn.com/editorial/videos/devotional_2.jpg",
        "explicit": false
      },
      {
        "id": "123",
        "name": "Travel",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/S6XP2BBIeO-station__20220317064742.jpg",
        "url": "https://www.jiosaavn.com/s/channel/travel/LTMMnGu9cbQ_",
        "badge": "",
        "subType": "mood",
        "isFeatured": false,
        "videoUrl": "http://c.saavncdn.com/editorial/videos/travel_3.mp4",
        "videoThumbnail": "https://c.saavncdn.com/editorial/videos/travel_2.jpg",
        "explicit": false
      },
      {
        "id": "126",
        "name": "Best of 2019",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/tPXW3ZnAVm-station__20220317082119.jpg",
        "url": "https://www.jiosaavn.com/s/channel/best-of-2019/g4uLYI7C9Nw_",
        "badge": "",
        "subType": "genre",
        "isFeatured": false,
        "videoUrl": "http://c.saavncdn.com/editorial/videos/2019_2.mp4",
        "videoThumbnail": "https://c.saavncdn.com/editorial/videos/best-of-2019_2.jpg",
        "explicit": false
      },
      {
        "id": "141",
        "name": "Decades of Romance",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/AtB5KfWL81-station__20220317082923.jpg",
        "url": "https://www.jiosaavn.com/s/channel/decades-of-romance/r6Nb69IIapI_",
        "badge": "",
        "subType": "genre",
        "isFeatured": false,
        "videoUrl": "http://c.saavncdn.com/editorial/videos/DecadesOfRomance_2.mp4",
        "videoThumbnail": "https://c.saavncdn.com/editorial/videos/decades-of-romance_2.jpg",
        "explicit": false
      },
      {
        "id": "21",
        "name": "Ghazal",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/7y31kg8rVP-station__20220317074544.jpg",
        "url": "https://www.jiosaavn.com/s/channel/ghazal/yj6euaS76uI_",
        "badge": "",
        "subType": "genre",
        "isFeatured": false,
        "videoUrl": "http://c.saavncdn.com/editorial/videos/Ghazal_2.mp4",
        "videoThumbnail": "https://c.saavncdn.com/editorial/videos/ghazal_2.jpg",
        "explicit": false
      },
      {
        "id": "72",
        "name": "Wedding",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/lbaFQZtBr2-station__20220317080724.jpg",
        "url": "https://www.jiosaavn.com/s/channel/wedding/icgz,,XXmAM_",
        "badge": "",
        "subType": "genre",
        "isFeatured": false,
        "videoUrl": "http://c.saavncdn.com/editorial/videos/wedding_3.mp4",
        "videoThumbnail": "https://c.saavncdn.com/editorial/videos/wedding_2.jpg",
        "explicit": false
      },
      {
        "id": "32",
        "name": "Sufi",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/SUwqXs7IZn-station__20220317074859.jpg",
        "url": "https://www.jiosaavn.com/s/channel/sufi/DOH-OPmC4AE_",
        "badge": "",
        "subType": "genre",
        "isFeatured": false,
        "videoUrl": "",
        "videoThumbnail": "",
        "explicit": false
      },
      {
        "id": "73",
        "name": "Kids",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/gmMO7wu4Ol-station__20220317063936.jpg",
        "url": "https://www.jiosaavn.com/s/channel/kids/7LSw,ctIuX8_",
        "badge": "",
        "subType": "genre",
        "isFeatured": false,
        "videoUrl": "",
        "videoThumbnail": "",
        "explicit": false
      },
      {
        "id": "15",
        "name": "Pop",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/K447BE46BH-station__20221007155849.jpg",
        "url": "https://www.jiosaavn.com/s/channel/pop/KLJ,xkx-IRo_",
        "badge": "",
        "subType": "genre",
        "isFeatured": false,
        "videoUrl": "",
        "videoThumbnail": "",
        "explicit": false
      },
      {
        "id": "139",
        "name": "Superstars",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/yO4ZEaVEu3-station__20220317071537.jpg",
        "url": "https://www.jiosaavn.com/s/channel/superstars/602Y4,xsT-U_",
        "badge": "",
        "subType": "mood",
        "isFeatured": false,
        "videoUrl": "",
        "videoThumbnail": "",
        "explicit": false
      },
      {
        "id": "69",
        "name": "Desi Hip Hop",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/PbsMCjANJr-station__20221007161910.jpg",
        "url": "https://www.jiosaavn.com/s/channel/desi-hip-hop/i7I4w9XrFA0_",
        "badge": "",
        "subType": "genre",
        "isFeatured": false,
        "videoUrl": "",
        "videoThumbnail": "",
        "explicit": false
      },
      {
        "id": "33",
        "name": "Alternative",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/CnMOsIC9mx-station__20220317063126.jpg",
        "url": "https://www.jiosaavn.com/s/channel/alternative/nmutII8VgGM_",
        "badge": "",
        "subType": "genre",
        "isFeatured": false,
        "videoUrl": "",
        "videoThumbnail": "",
        "explicit": false
      },
      {
        "id": "127",
        "name": "K-Pop",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/fAqqaGIh01-station__20220317082300.jpg",
        "url": "https://www.jiosaavn.com/s/channel/k-pop/qIz3FhjXRFI_",
        "badge": "",
        "subType": "genre",
        "isFeatured": false,
        "videoUrl": "",
        "videoThumbnail": "",
        "explicit": false
      },
      {
        "id": "132",
        "name": "Western Classical",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/eMne5eUbDD-station__20220317082714.jpg",
        "url": "https://www.jiosaavn.com/s/channel/western-classical/xLfB72H6IUY_",
        "badge": "",
        "subType": "genre",
        "isFeatured": false,
        "videoUrl": "",
        "videoThumbnail": "",
        "explicit": false
      },
      {
        "id": "142",
        "name": "Podsnacks - Podcasts under 10 mins",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/LnwJ93LSaT-station__20201020111200.jpg",
        "url": "https://www.jiosaavn.com/s/channel/podsnacks-podcasts-under-10-mins/Nakkh6hImTA_",
        "badge": "",
        "subType": "music_plus",
        "isFeatured": false,
        "videoUrl": "",
        "videoThumbnail": "",
        "explicit": false
      },
      {
        "id": "36",
        "name": "Rock",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/jyur9P7yeM-station__20221007155758.jpg",
        "url": "https://www.jiosaavn.com/s/channel/rock/oYOnCdZaXRQ_",
        "badge": "",
        "subType": "genre",
        "isFeatured": false,
        "videoUrl": "",
        "videoThumbnail": "",
        "explicit": false
      },
      {
        "id": "90",
        "name": "Fusion",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/pPGGhaVMuN-station__20220317080546.jpg",
        "url": "https://www.jiosaavn.com/s/channel/fusion/mMizSLX2ziI_",
        "badge": "",
        "subType": "genre",
        "isFeatured": false,
        "videoUrl": "",
        "videoThumbnail": "",
        "explicit": false
      },
      {
        "id": "47",
        "name": "Carnatic",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/GGFAgp9zRS-station__20220317080756.jpg",
        "url": "https://www.jiosaavn.com/s/channel/carnatic/hCk1Q5sEU,c_",
        "badge": "",
        "subType": "genre",
        "isFeatured": false,
        "videoUrl": "",
        "videoThumbnail": "",
        "explicit": false
      },
      {
        "id": "53",
        "name": "R&B",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/5i3naD2bOr-station__20220317080116.jpg",
        "url": "https://www.jiosaavn.com/s/channel/rb/I6t2BxDKGh0_",
        "badge": "",
        "subType": "genre",
        "isFeatured": false,
        "videoUrl": "",
        "videoThumbnail": "",
        "explicit": false
      },
      {
        "id": "66",
        "name": "Indie ",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/N51uSBp0oP-station__20220317063654.jpg",
        "url": "https://www.jiosaavn.com/s/channel/indie-/UTorEdGGhso_",
        "badge": "",
        "subType": "genre",
        "isFeatured": false,
        "videoUrl": "",
        "videoThumbnail": "",
        "explicit": false
      },
      {
        "id": "74",
        "name": "Folk",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/FhcVbbVIEf-station__20220317080619.jpg",
        "url": "https://www.jiosaavn.com/s/channel/folk/jhbeuMqB8c4_",
        "badge": "",
        "subType": "genre",
        "isFeatured": false,
        "videoUrl": "",
        "videoThumbnail": "",
        "explicit": false
      },
      {
        "id": "49",
        "name": "Hindustani Classical ",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/CeDJ2CWQvW-station__20201020121056.jpg",
        "url": "https://www.jiosaavn.com/s/channel/hindustani-classical-/7w0fhj-dMGw_",
        "badge": "",
        "subType": "genre",
        "isFeatured": false,
        "videoUrl": "",
        "videoThumbnail": "",
        "explicit": false
      },
      {
        "id": "28",
        "name": "Retro",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/3hCL8jIx43-station__20220317065138.jpg",
        "url": "https://www.jiosaavn.com/s/channel/retro/-woinCgq,qg_",
        "badge": "",
        "subType": "mood",
        "isFeatured": false,
        "videoUrl": "http://c.saavncdn.com/editorial/videos/Retro_2.mp4",
        "videoThumbnail": "https://c.saavncdn.com/editorial/videos/retro_2.jpg",
        "explicit": false
      },
      {
        "id": "149",
        "name": "Throwback Top 20",
        "subtitle": "",
        "type": "channel",
        "image": "https://c.saavncdn.com/editorial/NtR2fYAJsJ-station__20220317130333.jpg",
        "url": "https://www.jiosaavn.com/s/channel/throwback-top-20/kxYEuF8axGQ_",
        "badge": "",
        "subType": "genre",
        "isFeatured": false,
        "videoUrl": "",
        "videoThumbnail": "",
        "explicit": false
      }
    ],
    "charts": {
      "title": "Top Charts",
      "subtitle": "",
      "featuredText": "The top movers this week, from our editors.",
      "data": [
        {
          "id": "1134543272",
          "image": "https://c.saavncdn.com/editorial/Hindi-IndiaSuperhitsTop50_20230804105026.jpg",
          "name": "Hindi: India Superhits Top 50",
          "url": "https://www.jiosaavn.com/featured/hindi%3a-india-superhits-top-50/zlJfJYVuyjpxWb5,FqsjKg__",
          "type": "playlist",
          "count": 50,
          "listname": null,
          "language": null,
          "subtitle": null,
          "explicit": null,
          "firstname": null,
          "songCount": null
        },
        {
          "id": "1134595537",
          "image": "https://c.saavncdn.com/editorial/English-IndiaSuperhitsTop50_20230811090844.jpg",
          "name": "English: India Superhits Top 50",
          "url": "https://www.jiosaavn.com/featured/english%3a-india-superhits-top-50/aXoCADwITrUCObrEMJSxEw__",
          "type": "playlist",
          "count": 50,
          "listname": null,
          "language": null,
          "subtitle": null,
          "explicit": null,
          "firstname": null,
          "songCount": null
        },
        {
          "id": "1134548194",
          "image": "https://c.saavncdn.com/editorial/IndiaSuperhitsTop50_20230804092145.jpg",
          "name": "India Superhits Top 50",
          "url": "https://www.jiosaavn.com/featured/india-superhits-top-50/VuJUPQ9ch77bB,U5Yp5iAA__",
          "type": "playlist",
          "count": 50,
          "listname": null,
          "language": null,
          "subtitle": null,
          "explicit": null,
          "firstname": null,
          "songCount": null
        },
        {
          "id": "49",
          "image": "https://c.saavncdn.com/editorial/wt15-49_20230811093444.jpg?bch=1691956800",
          "name": "Weekly Top Songs",
          "url": "https://www.jiosaavn.com/featured/weekly-top-songs/8MT-LQlP35c_",
          "type": "playlist",
          "count": null,
          "listname": "weekly-top-songs",
          "language": null,
          "subtitle": null,
          "explicit": null,
          "firstname": null,
          "songCount": 30
        },
        {
          "id": "110858205",
          "image": "https://c.saavncdn.com/editorial/charts_TrendingToday_149406_20220319164713.jpg",
          "name": "Trending Today",
          "url": "https://www.jiosaavn.com/featured/trending_today/I3kvhipIy73uCJW60TJk1Q__",
          "type": "playlist",
          "count": null,
          "listname": null,
          "language": "hindi",
          "subtitle": "JioSaavn",
          "explicit": false,
          "firstname": "JioSaavn",
          "songCount": null
        },
        {
          "id": "142311984",
          "image": "https://c.saavncdn.com/editorial/charts_RomanticTop40_167985_20220311173413.jpg",
          "name": "Romantic Top 40",
          "url": "https://www.jiosaavn.com/featured/romantic_top_40/m9Qkal5S733ufxkxMEIbIw__",
          "type": "playlist",
          "count": null,
          "listname": null,
          "language": "hindi",
          "subtitle": "JioSaavn",
          "explicit": false,
          "firstname": "JioSaavn",
          "songCount": null
        },
        {
          "id": "845149969",
          "image": "https://c.saavncdn.com/editorial/charts_RomanticTop40-English_158417_20220315153203.jpg",
          "name": "Romantic Top 40 -  English",
          "url": "https://www.jiosaavn.com/featured/romantic_top_40____english/jVmOAc1aK2OO0eMLZZxqsA__",
          "type": "playlist",
          "count": null,
          "listname": null,
          "language": "english",
          "subtitle": "JioSaavn",
          "explicit": false,
          "firstname": "JioSaavn",
          "songCount": null
        },
        {
          "id": "1167751270",
          "image": "https://c.saavncdn.com/editorial/charts_CHARTS_SAAVN_hindi_2000s_139784_20230711094322.jpg",
          "name": "Hindi 2000s",
          "url": "https://www.jiosaavn.com/featured/hindi_2000s/dSYq41esdPJwtkLw7-JlUw__",
          "type": "playlist",
          "count": null,
          "listname": null,
          "language": "hindi",
          "subtitle": "JioSaavn",
          "explicit": false,
          "firstname": "JioSaavn",
          "songCount": null
        },
        {
          "id": "61969868",
          "image": "https://c.saavncdn.com/editorial/charts_InternationalCharts_143799_20220315153513.jpg",
          "name": "International Charts",
          "url": "https://www.jiosaavn.com/featured/international_charts/DjLfyo0wfbk_",
          "type": "playlist",
          "count": null,
          "listname": null,
          "language": "english",
          "subtitle": "JioSaavn",
          "explicit": false,
          "firstname": "JioSaavn",
          "songCount": null
        },
        {
          "id": "1167751266",
          "image": "https://c.saavncdn.com/editorial/charts_CHARTS_SAAVN_hindi_1990s_175982_20230711094900.jpg",
          "name": "Hindi 1990s",
          "url": "https://www.jiosaavn.com/featured/hindi_1990s/dSYq41esdPJAI5VmDfZSSg__",
          "type": "playlist",
          "count": null,
          "listname": null,
          "language": "hindi",
          "subtitle": "JioSaavn",
          "explicit": false,
          "firstname": "JioSaavn",
          "songCount": null
        },
        {
          "id": "142311985",
          "image": "https://c.saavncdn.com/editorial/charts_EDMHot40_150598_20220315153607.jpg",
          "name": "EDM Hot 40",
          "url": "https://www.jiosaavn.com/featured/edm_hot_40/m9Qkal5S733uCJW60TJk1Q__",
          "type": "playlist",
          "count": null,
          "listname": null,
          "language": "english",
          "subtitle": "JioSaavn",
          "explicit": false,
          "firstname": "JioSaavn",
          "songCount": null
        },
        {
          "id": "1170578723",
          "image": "https://c.saavncdn.com/editorial/charts_CHARTS_SAAVN_hindi_1980s_139462_20230711095004.jpg",
          "name": "Hindi 1980s",
          "url": "https://www.jiosaavn.com/featured/hindi_1980s/,H6xc4W4ZikV3Xpvr9dnYw__",
          "type": "playlist",
          "count": null,
          "listname": null,
          "language": "hindi",
          "subtitle": "JioSaavn",
          "explicit": false,
          "firstname": "JioSaavn",
          "songCount": null
        },
        {
          "id": "63116921",
          "image": "https://c.saavncdn.com/editorial/charts_English00s_177292_20220315153709.jpg",
          "name": "English 2000s",
          "url": "https://www.jiosaavn.com/featured/english_2000s/ez5bY2zH0CQ_",
          "type": "playlist",
          "count": null,
          "listname": null,
          "language": "english",
          "subtitle": "JioSaavn",
          "explicit": false,
          "firstname": "JioSaavn",
          "songCount": null
        },
        {
          "id": "114107255",
          "image": "https://c.saavncdn.com/editorial/charts_Hindi1970s_180599_20230713045713.jpg",
          "name": "Hindi 1970s",
          "url": "https://www.jiosaavn.com/featured/hindi_1970s/Pgp0Amd0LZnuCJW60TJk1Q__",
          "type": "playlist",
          "count": null,
          "listname": null,
          "language": "hindi",
          "subtitle": "JioSaavn",
          "explicit": false,
          "firstname": "JioSaavn",
          "songCount": null
        },
        {
          "id": "63116918",
          "image": "https://c.saavncdn.com/editorial/charts_English90s_183339_20220315153803.jpg",
          "name": "English 1990s",
          "url": "https://www.jiosaavn.com/featured/english_1990s/H2FgDdeBnNg_",
          "type": "playlist",
          "count": null,
          "listname": null,
          "language": "english",
          "subtitle": "JioSaavn",
          "explicit": false,
          "firstname": "JioSaavn",
          "songCount": null
        },
        {
          "id": "901538745",
          "image": "https://c.saavncdn.com/editorial/charts_English80s_102505_20220315153947.jpg",
          "name": "English 1980s",
          "url": "https://www.jiosaavn.com/featured/english_1980s/FAbXU36a33nuCJW60TJk1Q__",
          "type": "playlist",
          "count": null,
          "listname": null,
          "language": "english",
          "subtitle": "JioSaavn",
          "explicit": false,
          "firstname": "JioSaavn",
          "songCount": null
        },
        {
          "id": "901538742",
          "image": "https://c.saavncdn.com/editorial/charts_English70s_136461_20220315154102.jpg",
          "name": "English 1970s",
          "url": "https://www.jiosaavn.com/featured/english_1970s/FAbXU36a33nc1EngHtQQ2g__",
          "type": "playlist",
          "count": null,
          "listname": null,
          "language": "english",
          "subtitle": "JioSaavn",
          "explicit": false,
          "firstname": "JioSaavn",
          "songCount": null
        },
        {
          "id": "901538738",
          "image": "https://c.saavncdn.com/editorial/charts_English60s_154284_20220315154214.jpg",
          "name": "English 1960s",
          "url": "https://www.jiosaavn.com/featured/english_1960s/DlapWxJHOA9Fo9wdEAzFBA__",
          "type": "playlist",
          "count": null,
          "listname": null,
          "language": "english",
          "subtitle": "JioSaavn",
          "explicit": false,
          "firstname": "JioSaavn",
          "songCount": null
        },
        {
          "id": "82711685",
          "image": "https://c.saavncdn.com/editorial/charts_EnglishRetro_174284_20220315154327.jpg",
          "name": "English Retro",
          "url": "https://www.jiosaavn.com/featured/english_retro/zp3o7OIx1fo_",
          "type": "playlist",
          "count": null,
          "listname": null,
          "language": "english",
          "subtitle": "JioSaavn",
          "explicit": false,
          "firstname": "JioSaavn",
          "songCount": null
        },
        {
          "id": "63116930",
          "image": "https://c.saavncdn.com/editorial/charts_English2010s_20220316070529.jpg",
          "name": "English 2010s",
          "url": "https://www.jiosaavn.com/featured/english_2010s/9J4ePDXBp8k_",
          "type": "playlist",
          "count": null,
          "listname": null,
          "language": "english",
          "subtitle": "JioSaavn",
          "explicit": false,
          "firstname": "JioSaavn",
          "songCount": null
        }
      ]
    },
    "cityMod": {
      "title": "What's Hot In Hapur",
      "subtitle": "Trending This Week",
      "featuredText": null,
      "data": [
        {
          "id": "81Jj5__k",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/756/Banke-Bihari-Teri-Aarti-Gaun-Hindi-2022-20220509112838-50x50.webp"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/756/Banke-Bihari-Teri-Aarti-Gaun-Hindi-2022-20220509112838-150x150.webp"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/756/Banke-Bihari-Teri-Aarti-Gaun-Hindi-2022-20220509112838-500x500.webp"
            }
          ],
          "url": "https://www.jiosaavn.com/song/banke-bihari-teri-aarti-gaun/SFkhW0FvaFg",
          "subtitle": "",
          "name": "Banke Bihari Teri Aarti Gaun",
          "type": "song",
          "explicit": false,
          "query": null,
          "albumId": "34846728",
          "featuredStationType": null,
          "multipleTunes": [
            {
              "id": "010911261571709",
              "subtype": "songTune",
              "name": "Banke Bihari Teri Aarti Gaun",
              "type": "jiotune"
            }
          ]
        },
        {
          "id": "",
          "image": "https://c.saavncdn.com/artists/Lata_Mangeshkar_004_20230623105323.jpg",
          "url": "",
          "subtitle": "",
          "name": "Lata Mangeshkar",
          "type": "radio_station",
          "explicit": false,
          "query": "Lata Mangeshkar",
          "albumId": null,
          "featuredStationType": "artist",
          "multipleTunes": null
        },
        {
          "id": "",
          "image": "https://c.saavncdn.com/artists/Alka_Yagnik_002_20220314192930.jpg",
          "url": "",
          "subtitle": "",
          "name": "Alka Yagnik",
          "type": "radio_station",
          "explicit": false,
          "query": "Alka Yagnik",
          "albumId": null,
          "featuredStationType": "artist",
          "multipleTunes": null
        },
        {
          "id": "dEeyK9Hx",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/538/Shri-Ramchandra-Kripalu-Bhajuman-Hindi-2021-20211212153036-50x50.webp"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/538/Shri-Ramchandra-Kripalu-Bhajuman-Hindi-2021-20211212153036-150x150.webp"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/538/Shri-Ramchandra-Kripalu-Bhajuman-Hindi-2021-20211212153036-500x500.webp"
            }
          ],
          "url": "https://www.jiosaavn.com/song/shri-ramchandra-kripalu-bhajuman/FC0OSD8Jf0s",
          "subtitle": "",
          "name": "Shri Ramchandra Kripalu Bhajuman",
          "type": "song",
          "explicit": false,
          "query": null,
          "albumId": "31483861",
          "featuredStationType": null,
          "multipleTunes": [
            {
              "id": "010911261476626",
              "subtype": "songTune",
              "name": "Shri Ramchandra Kripalu Bhajuman",
              "type": "jiotune"
            }
          ]
        },
        {
          "id": "TpnAIhVT",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/427/Shiv-Dhun-Om-Namah-Shivaaya-Sanskrit-2022-20220714114444-50x50.webp"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/427/Shiv-Dhun-Om-Namah-Shivaaya-Sanskrit-2022-20220714114444-150x150.webp"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/427/Shiv-Dhun-Om-Namah-Shivaaya-Sanskrit-2022-20220714114444-500x500.webp"
            }
          ],
          "url": "https://www.jiosaavn.com/song/shiv-dhun-om-namah-shivaaya/JBgFcD1YYWc",
          "subtitle": "",
          "name": "Shiv Dhun Om Namah Shivaaya",
          "type": "song",
          "explicit": false,
          "query": null,
          "albumId": "36482012",
          "featuredStationType": null,
          "multipleTunes": [
            {
              "id": "010911261624921",
              "subtype": "songTune",
              "name": "Shiv Dhun Om Namah Shivaaya",
              "type": "jiotune"
            }
          ]
        },
        {
          "id": "StqdNUJn",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/567/Kaalbhairav-Ashtakam-Sanskrit-2022-20220601172933-50x50.webp"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/567/Kaalbhairav-Ashtakam-Sanskrit-2022-20220601172933-150x150.webp"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/567/Kaalbhairav-Ashtakam-Sanskrit-2022-20220601172933-500x500.webp"
            }
          ],
          "url": "https://www.jiosaavn.com/song/kaalbhairav-ashtakam/IxwaVTplfV0",
          "subtitle": "",
          "name": "Kaalbhairav Ashtakam",
          "type": "song",
          "explicit": false,
          "query": null,
          "albumId": "35413056",
          "featuredStationType": null,
          "multipleTunes": [
            {
              "id": "010911261588064",
              "subtype": "songTune",
              "name": "Kaalbhairav Ashtakam",
              "type": "jiotune"
            }
          ]
        },
        {
          "id": "",
          "image": "https://c.saavncdn.com/artists/Kumar_Sanu.jpg",
          "url": "",
          "subtitle": "",
          "name": "Kumar Sanu",
          "type": "radio_station",
          "explicit": false,
          "query": "Kumar Sanu",
          "albumId": null,
          "featuredStationType": "artist",
          "multipleTunes": null
        },
        {
          "id": "cSvw-TOk",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/550/Lingashtakam-Brahmamurari-Surarchitalingam-Sanskrit-2022-20220601172956-50x50.webp"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/550/Lingashtakam-Brahmamurari-Surarchitalingam-Sanskrit-2022-20220601172956-150x150.webp"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/550/Lingashtakam-Brahmamurari-Surarchitalingam-Sanskrit-2022-20220601172956-500x500.webp"
            }
          ],
          "url": "https://www.jiosaavn.com/song/lingashtakam-brahmamurari-surarchitalingam/EzsdRllkeFg",
          "subtitle": "",
          "name": "Lingashtakam Brahmamurari Surarchitalingam",
          "type": "song",
          "explicit": false,
          "query": null,
          "albumId": "35413027",
          "featuredStationType": null,
          "multipleTunes": [
            {
              "id": "010911261588063",
              "subtype": "songTune",
              "name": "Lingashtakam Brahmamurari Surarchitalingam",
              "type": "jiotune"
            }
          ]
        },
        {
          "id": "",
          "image": "https://c.saavncdn.com/artists/Udit_Narayan.jpg",
          "url": "",
          "subtitle": "",
          "name": "Udit Narayan",
          "type": "radio_station",
          "explicit": false,
          "query": "Udit Narayan",
          "albumId": null,
          "featuredStationType": "artist",
          "multipleTunes": null
        },
        {
          "id": "952419461",
          "image": "https://c.saavncdn.com/editorial/IndipopLoveSongs_20230725171152.jpg",
          "url": "https://www.jiosaavn.com/featured/indipop-love-songs/MznOa3aaas4GSw2I1RxdhQ__",
          "subtitle": "Arijit Singh, Dulquer Salmaan, Jasleen Royal, and more",
          "name": "Indipop Love Songs",
          "type": "playlist",
          "explicit": false,
          "query": null,
          "albumId": null,
          "featuredStationType": null,
          "multipleTunes": null
        },
        {
          "id": "",
          "image": "https://c.saavncdn.com/artists/Yo_Yo_Honey_Singh_002_20221216102650.jpg",
          "url": "",
          "subtitle": "",
          "name": "Yo Yo Honey Singh",
          "type": "radio_station",
          "explicit": false,
          "query": "Yo Yo Honey Singh",
          "albumId": null,
          "featuredStationType": "artist",
          "multipleTunes": null
        },
        {
          "id": "CI-Lpmsd",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/155/Ashutosh-Shashank-Shekhar-Hindi-2021-20211209162847-50x50.webp"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/155/Ashutosh-Shashank-Shekhar-Hindi-2021-20211209162847-150x150.webp"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/155/Ashutosh-Shashank-Shekhar-Hindi-2021-20211209162847-500x500.webp"
            }
          ],
          "url": "https://www.jiosaavn.com/song/ashutosh-shashank-shekhar/MyFGfQRdRFc",
          "subtitle": "",
          "name": "Ashutosh Shashank Shekhar",
          "type": "song",
          "explicit": false,
          "query": null,
          "albumId": "31412642",
          "featuredStationType": null,
          "multipleTunes": [
            {
              "id": "010911261475143",
              "subtype": "songTune",
              "name": "Ashutosh Shashank Shekhar",
              "type": "jiotune"
            }
          ]
        },
        {
          "id": "154589446",
          "image": "https://c.saavncdn.com/editorial/DilDaNiMadaSidhuMooseWala_20221107063722.jpg",
          "url": "https://www.jiosaavn.com/featured/lets-play-sidhu-moose-wala-punjabi/qkut1WxZp,rfemJ68FuXsA__",
          "subtitle": "Sidhu Moose Wala, Burna Boy, Steel Banglez, and more",
          "name": "Let's Play - Sidhu Moose Wala - Punjabi",
          "type": "playlist",
          "explicit": false,
          "query": null,
          "albumId": null,
          "featuredStationType": null,
          "multipleTunes": null
        },
        {
          "id": "30793386",
          "image": "https://c.saavncdn.com/editorial/Let_sPlayDiljitDosanjh_20221017123730.jpg",
          "url": "https://www.jiosaavn.com/featured/lets-play-diljit-dosanjh-punjabi/0kQshFJLbtQ_",
          "subtitle": "Diljit Dosanjh",
          "name": "Let's Play - Diljit Dosanjh - Punjabi",
          "type": "playlist",
          "explicit": false,
          "query": null,
          "albumId": null,
          "featuredStationType": null,
          "multipleTunes": null
        },
        {
          "id": "784453101",
          "image": "https://c.saavncdn.com/editorial/logo/JioTunesHeartbreakHitsHindi_20210208181900.jpg",
          "url": "https://www.jiosaavn.com/featured/heartbreak-hits-hindi-jiotunes/gJm,sGbLZpsGSw2I1RxdhQ__",
          "subtitle": "B Praak, Vishal Mishra, and Darshan Raval",
          "name": "Heartbreak Hits - Hindi - JioTunes",
          "type": "playlist",
          "explicit": false,
          "query": null,
          "albumId": null,
          "featuredStationType": null,
          "multipleTunes": null
        },
        {
          "id": "170968270",
          "image": "https://c.saavncdn.com/editorial/logo/TopJioTunesBollywoodRomanticHindi_20210208181852.jpg",
          "url": "https://www.jiosaavn.com/featured/bollywood-romantic-hindi-top-jiotunes/3XQBTVdiJc5uOxiEGmm6lQ__",
          "subtitle": "by JioSaavn",
          "name": "Bollywood Romantic - Hindi - Top JioTunes",
          "type": "playlist",
          "explicit": false,
          "query": null,
          "albumId": null,
          "featuredStationType": null,
          "multipleTunes": null
        },
        {
          "id": "",
          "image": "https://c.saavncdn.com/artists/Khasa_Aala_Chahar_001_20230313111956.jpg",
          "url": "",
          "subtitle": "",
          "name": "Khasa Aala Chahar",
          "type": "radio_station",
          "explicit": false,
          "query": "Khasa Aala Chahar",
          "albumId": null,
          "featuredStationType": "artist",
          "multipleTunes": null
        },
        {
          "id": "",
          "image": "https://c.saavncdn.com/artists/Kajal_Maheriya_20190709113326.jpg",
          "url": "",
          "subtitle": "",
          "name": "Kajal Maheriya",
          "type": "radio_station",
          "explicit": false,
          "query": "Kajal Maheriya",
          "albumId": null,
          "featuredStationType": "artist",
          "multipleTunes": null
        },
        {
          "id": "970907476",
          "image": "https://c.saavncdn.com/editorial/artist_hits-970907502_20220912064634.jpg",
          "url": "https://www.jiosaavn.com/featured/artist-hits-amrita-chaturvedi/f96b,NVyseDfemJ68FuXsA__",
          "subtitle": "by JioSaavn",
          "name": "Artist Hits - Amrita Chaturvedi",
          "type": "playlist",
          "explicit": false,
          "query": null,
          "albumId": null,
          "featuredStationType": null,
          "multipleTunes": null
        },
        {
          "id": "NZWK5jkY",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/152/Hari-Sundar-Nand-Mukund-Hindi-2022-20220516212421-50x50.webp"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/152/Hari-Sundar-Nand-Mukund-Hindi-2022-20220516212421-150x150.webp"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/152/Hari-Sundar-Nand-Mukund-Hindi-2022-20220516212421-500x500.webp"
            }
          ],
          "url": "https://www.jiosaavn.com/song/hari-sundar-nand-mukund/PjI8ekFaXGo",
          "subtitle": "",
          "name": "Hari Sundar Nand Mukund",
          "type": "song",
          "explicit": false,
          "query": null,
          "albumId": "35019437",
          "featuredStationType": null,
          "multipleTunes": [
            {
              "id": "010911261575664",
              "subtype": "songTune",
              "name": "Hari Sundar Nand Mukund",
              "type": "jiotune"
            }
          ]
        },
        {
          "id": "",
          "image": "https://c.saavncdn.com/artists/Emraan_Hashmi_002_20230228120108.jpg",
          "url": "",
          "subtitle": "",
          "name": "Emraan Hashmi",
          "type": "radio_station",
          "explicit": false,
          "query": "Emraan Hashmi",
          "albumId": null,
          "featuredStationType": "artist",
          "multipleTunes": null
        },
        {
          "id": "C_rA6Nl6",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/721/Jai-Ganesh-Jai-Ganesh-Deva-Hindi-2022-20220416123541-50x50.webp"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/721/Jai-Ganesh-Jai-Ganesh-Deva-Hindi-2022-20220416123541-150x150.webp"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/721/Jai-Ganesh-Jai-Ganesh-Deva-Hindi-2022-20220416123541-500x500.webp"
            }
          ],
          "url": "https://www.jiosaavn.com/song/jai-ganesh-jai-ganesh-deva/MzcZcEJ,WwU",
          "subtitle": "",
          "name": "Jai Ganesh Jai Ganesh Deva",
          "type": "song",
          "explicit": false,
          "query": null,
          "albumId": "34308071",
          "featuredStationType": null,
          "multipleTunes": [
            {
              "id": "010911261558287",
              "subtype": "songTune",
              "name": "Jai Ganesh Jai Ganesh Deva",
              "type": "jiotune"
            }
          ]
        }
      ]
    },
    "globalConfig": {
      "random_songs_listid": {
        "english": {
          "title": null,
          "count": 1007,
          "image": "https://pli.saavncdn.com/23/21/2212321.jpg",
          "listid": "2212321"
        },
        "hindi": {
          "title": null,
          "count": 787,
          "image": "https://pli.saavncdn.com/54/24/5424.jpg",
          "listid": "5424"
        }
      },
      "weekly_top_songs_listid": {
        "english": {
          "title": "Weekly Top Songs",
          "count": 30,
          "image": "https://c.saavncdn.com/editorial/wt15-7386899_20230811092534.jpg?bch=1691956800",
          "listid": "7386899"
        },
        "hindi": {
          "title": "Weekly Top Songs",
          "count": 30,
          "image": "https://c.saavncdn.com/editorial/wt15-49_20230811093444.jpg?bch=1691956800",
          "listid": "49"
        }
      }
    },
    "albums": {
      "title": "New Releases",
      "subtitle": "",
      "featuredText": null,
      "data": [
        {
          "id": "jyXnjyBl",
          "name": "bad idea right?",
          "subtitle": "Olivia Rodrigo - bad idea right?",
          "type": "song",
          "language": "english",
          "playCount": 7125,
          "explicit": false,
          "year": 2023,
          "url": "https://www.jiosaavn.com/song/bad-idea-right/GhEzXx5JdV8",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/267/bad-idea-right-English-2023-20230811063922-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/267/bad-idea-right-English-2023-20230811063922-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/267/bad-idea-right-English-2023-20230811063922-500x500.jpg"
            }
          ],
          "releaseDate": "2023-08-11",
          "artistMap": {
            "primaryArtists": [
              {
                "id": "2255753",
                "name": "Olivia Rodrigo",
                "role": "singer",
                "image": "https://c.saavncdn.com/064/All-I-Want-From-High-School-Musical-The-Musical-The-Series--English-2019-20191125230736-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/olivia-rodrigo-songs/B6E0EBxDcT4_"
              }
            ],
            "featuredArtists": [],
            "artists": [
              {
                "id": "716139",
                "name": "Daniel Nigro",
                "role": "",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/daniel-nigro-songs/-MQQztYc7bQ_"
              },
              {
                "id": "2255753",
                "name": "Olivia Rodrigo",
                "role": "singer",
                "image": "https://c.saavncdn.com/064/All-I-Want-From-High-School-Musical-The-Musical-The-Series--English-2019-20191125230736-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/olivia-rodrigo-songs/B6E0EBxDcT4_"
              }
            ]
          },
          "songCount": null,
          "copyrightText": "℗ 2023 Olivia Rodrigo, under exclusive license to Geffen Records",
          "isDolbyContent": false,
          "songs": []
        },
        {
          "id": "47439200",
          "name": "Dil Ka Telephone 2.0 (From \"Dream Girl 2\")",
          "subtitle": "",
          "type": "album",
          "language": "hindi",
          "playCount": 0,
          "explicit": false,
          "year": 0,
          "url": "https://www.jiosaavn.com/album/dil-ka-telephone-2.0-from-dream-girl-2/s0N1ifyNvXc_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/649/Dil-Ka-Telephone-2-0-From-Dream-Girl-2-Hindi-2023-20230810125949-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/649/Dil-Ka-Telephone-2-0-From-Dream-Girl-2-Hindi-2023-20230810125949-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/649/Dil-Ka-Telephone-2-0-From-Dream-Girl-2-Hindi-2023-20230810125949-500x500.jpg"
            }
          ],
          "releaseDate": "2023-08-10",
          "artistMap": {
            "primaryArtists": [],
            "featuredArtists": [],
            "artists": [
              {
                "id": "890048",
                "name": "Meet Bros",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/meet-bros-songs/sUhZsB99Ezg_"
              },
              {
                "id": "672010",
                "name": "Jonita Gandhi",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/jonita-gandhi-songs/2deuXsoZiWA_"
              },
              {
                "id": "881158",
                "name": "Jubin Nautiyal",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/jubin-nautiyal-songs/uGdfg6zGf4s_"
              }
            ]
          },
          "songCount": 0,
          "copyrightText": null,
          "isDolbyContent": null,
          "songs": []
        },
        {
          "id": "47279722",
          "name": "Gadar 2",
          "subtitle": "",
          "type": "album",
          "language": "hindi",
          "playCount": 0,
          "explicit": false,
          "year": 0,
          "url": "https://www.jiosaavn.com/album/gadar-2/zKU0zGJl3GY_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/716/Gadar-2-Hindi-2023-20230805095149-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/716/Gadar-2-Hindi-2023-20230805095149-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/716/Gadar-2-Hindi-2023-20230805095149-500x500.jpg"
            }
          ],
          "releaseDate": "2023-08-05",
          "artistMap": {
            "primaryArtists": [],
            "featuredArtists": [],
            "artists": [
              {
                "id": "702592",
                "name": "Mithoon",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/mithoon-songs/nQKQiNRsTKs_"
              },
              {
                "id": "461323",
                "name": "Uttam Singh",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/uttam-singh-songs/oNUhFwf,KLc_"
              }
            ]
          },
          "songCount": 0,
          "copyrightText": null,
          "isDolbyContent": null,
          "songs": []
        },
        {
          "id": "53DIrI2v",
          "name": "Sleepless Nights",
          "subtitle": "Armaan Malik - Sleepless Nights",
          "type": "song",
          "language": "english",
          "playCount": 152336,
          "explicit": false,
          "year": 2023,
          "url": "https://www.jiosaavn.com/song/sleepless-nights/RVsveAZ5BUU",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/005/Sleepless-Nights-English-2023-20230720172322-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/005/Sleepless-Nights-English-2023-20230720172322-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/005/Sleepless-Nights-English-2023-20230720172322-500x500.jpg"
            }
          ],
          "releaseDate": "2023-08-04",
          "artistMap": {
            "primaryArtists": [
              {
                "id": "464656",
                "name": "Armaan Malik",
                "role": "singer",
                "image": "https://c.saavncdn.com/artists/Armaan_Malik_003_20220107135753_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/armaan-malik-songs/1iZ7Gi0bi1Y_"
              }
            ],
            "featuredArtists": [],
            "artists": [
              {
                "id": "464656",
                "name": "Armaan Malik",
                "role": "singer",
                "image": "https://c.saavncdn.com/artists/Armaan_Malik_003_20220107135753_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/armaan-malik-songs/1iZ7Gi0bi1Y_"
              },
              {
                "id": "2737681",
                "name": "Natania",
                "role": "music",
                "image": "https://c.saavncdn.com/artists/Natania_Lalwani_001_20191128084944_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/natania-songs/08-ViIgdTvw_"
              }
            ]
          },
          "songCount": null,
          "copyrightText": "© 2023 Always Music Global",
          "isDolbyContent": false,
          "songs": []
        },
        {
          "id": "47339180",
          "name": "OMG 2",
          "subtitle": "",
          "type": "album",
          "language": "hindi",
          "playCount": 0,
          "explicit": false,
          "year": 0,
          "url": "https://www.jiosaavn.com/album/omg-2/t5XBxIwvpms_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/526/OMG-2-Hindi-2023-20230808131515-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/526/OMG-2-Hindi-2023-20230808131515-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/526/OMG-2-Hindi-2023-20230808131515-500x500.jpg"
            }
          ],
          "releaseDate": "2023-08-08",
          "artistMap": {
            "primaryArtists": [],
            "featuredArtists": [],
            "artists": [
              {
                "id": "3436900",
                "name": "Hansraj Raghuwanshi",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/hansraj-raghuwanshi-songs/sF6m,UAR8co_"
              },
              {
                "id": "11036206",
                "name": "DJStrings",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/djstrings-songs/Ay-hdRA3gcQ_"
              },
              {
                "id": "456478",
                "name": "Raahi",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/raahi-songs/-TPOcVd8A4k_"
              },
              {
                "id": "2101009",
                "name": "Vikram Montrose",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/vikram-montrose-songs/EAV7PA5PX3Q_"
              },
              {
                "id": "3531188",
                "name": "Pranaay",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/pranaay-songs/jzeeLby6orI_"
              },
              {
                "id": "459406",
                "name": "Sandesh Shandilya",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/sandesh-shandilya-songs/45Tsjp,jYm8_"
              }
            ]
          },
          "songCount": 0,
          "copyrightText": null,
          "isDolbyContent": null,
          "songs": []
        },
        {
          "id": "47472443",
          "name": "Layover",
          "subtitle": "",
          "type": "album",
          "language": "english",
          "playCount": 0,
          "explicit": false,
          "year": 0,
          "url": "https://www.jiosaavn.com/album/layover/,S0pWMDxjGQ_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/403/Layover-English-2023-20230811093146-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/403/Layover-English-2023-20230811093146-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/403/Layover-English-2023-20230811093146-500x500.jpg"
            }
          ],
          "releaseDate": "2023-08-11",
          "artistMap": {
            "primaryArtists": [],
            "featuredArtists": [],
            "artists": [
              {
                "id": "838690",
                "name": "V",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/v-songs/r11IdHYX7Jw_"
              }
            ]
          },
          "songCount": 0,
          "copyrightText": null,
          "isDolbyContent": null,
          "songs": []
        },
        {
          "id": "47089165",
          "name": "UTOPIA",
          "subtitle": "",
          "type": "album",
          "language": "english",
          "playCount": 0,
          "explicit": true,
          "year": 0,
          "url": "https://www.jiosaavn.com/album/utopia/mdvHsyg4XNY_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/882/UTOPIA-English-2023-20230728085013-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/882/UTOPIA-English-2023-20230728085013-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/882/UTOPIA-English-2023-20230728085013-500x500.jpg"
            }
          ],
          "releaseDate": "2023-07-28",
          "artistMap": {
            "primaryArtists": [],
            "featuredArtists": [],
            "artists": [
              {
                "id": "614647",
                "name": "Travis Scott",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/travis-scott-songs/n7hZTK-39Zo_"
              }
            ]
          },
          "songCount": 0,
          "copyrightText": null,
          "isDolbyContent": null,
          "songs": []
        },
        {
          "id": "47135052",
          "name": "Zinda Banda (From \"Jawan\")",
          "subtitle": "",
          "type": "album",
          "language": "hindi",
          "playCount": 0,
          "explicit": false,
          "year": 0,
          "url": "https://www.jiosaavn.com/album/zinda-banda-from-jawan/WiQJ2zXJX2w_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/406/Zinda-Banda-From-Jawan-Hindi-2023-20230731043357-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/406/Zinda-Banda-From-Jawan-Hindi-2023-20230731043357-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/406/Zinda-Banda-From-Jawan-Hindi-2023-20230731043357-500x500.jpg"
            }
          ],
          "releaseDate": "2023-07-31",
          "artistMap": {
            "primaryArtists": [],
            "featuredArtists": [],
            "artists": [
              {
                "id": "455663",
                "name": "Anirudh Ravichander",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/anirudh-ravichander-songs/lBzQd8c-xCY_"
              },
              {
                "id": "456259",
                "name": "Irshad Kamil",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/irshad-kamil-songs/vgKnepfCTXw_"
              },
              {
                "id": "456259",
                "name": "Irshad Kamil",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/irshad-kamil-songs/vgKnepfCTXw_"
              },
              {
                "id": "455663",
                "name": "Anirudh Ravichander",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/anirudh-ravichander-songs/lBzQd8c-xCY_"
              },
              {
                "id": "455663",
                "name": "Anirudh Ravichander",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/anirudh-ravichander-songs/lBzQd8c-xCY_"
              },
              {
                "id": "456259",
                "name": "Irshad Kamil",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/irshad-kamil-songs/vgKnepfCTXw_"
              }
            ]
          },
          "songCount": 0,
          "copyrightText": null,
          "isDolbyContent": null,
          "songs": []
        },
        {
          "id": "47423901",
          "name": "Ghoomer",
          "subtitle": "",
          "type": "album",
          "language": "hindi",
          "playCount": 0,
          "explicit": false,
          "year": 0,
          "url": "https://www.jiosaavn.com/album/ghoomer/T2xhntg6l08_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/748/Ghoomer-Hindi-2023-20230809190843-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/748/Ghoomer-Hindi-2023-20230809190843-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/748/Ghoomer-Hindi-2023-20230809190843-500x500.jpg"
            }
          ],
          "releaseDate": "2023-08-10",
          "artistMap": {
            "primaryArtists": [],
            "featuredArtists": [],
            "artists": [
              {
                "id": "457422",
                "name": "Amit Trivedi",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/amit-trivedi-songs/BxN3kFKSp1o_"
              }
            ]
          },
          "songCount": 0,
          "copyrightText": null,
          "isDolbyContent": null,
          "songs": []
        },
        {
          "id": "b3zsDV-V",
          "name": "Paint The Town Red",
          "subtitle": "Doja Cat - Paint The Town Red",
          "type": "song",
          "language": "english",
          "playCount": 20876,
          "explicit": true,
          "year": 2023,
          "url": "https://www.jiosaavn.com/song/paint-the-town-red/ElsRQjBmGmU",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/104/Paint-The-Town-Red-English-2023-20230728230348-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/104/Paint-The-Town-Red-English-2023-20230728230348-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/104/Paint-The-Town-Red-English-2023-20230728230348-500x500.jpg"
            }
          ],
          "releaseDate": "2023-08-04",
          "artistMap": {
            "primaryArtists": [
              {
                "id": "697799",
                "name": "Doja Cat",
                "role": "singer",
                "image": "https://c.saavncdn.com/artists/Doja_Cat_20200218145042_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/doja-cat-songs/GuwOruHLpLk_"
              }
            ],
            "featuredArtists": [],
            "artists": [
              {
                "id": "697800",
                "name": "Amala Zandile Dlamini",
                "role": "",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/amala-zandile-dlamini-songs/2vDxJiTJILg_"
              },
              {
                "id": "2668471",
                "name": "Isaac Earl Bynum",
                "role": "",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/isaac-earl-bynum-songs/x4Gp,y51ExQ_"
              },
              {
                "id": "511768",
                "name": "Burt Bacharach",
                "role": "",
                "image": "https://c.saavncdn.com/920/Butch-Cassidy-And-The-Sundance-Kid-1999-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/burt-bacharach-songs/ji0bHhG9aOg_"
              },
              {
                "id": "512614",
                "name": "Hal David",
                "role": "music",
                "image": "https://c.saavncdn.com/002/Stage-Screen-The-New-Standards-English-2013-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/hal-david-songs/fT9dBdwnFGM_"
              },
              {
                "id": "697799",
                "name": "Doja Cat",
                "role": "singer",
                "image": "https://c.saavncdn.com/artists/Doja_Cat_20200218145042_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/doja-cat-songs/GuwOruHLpLk_"
              }
            ]
          },
          "songCount": null,
          "copyrightText": "(P) 2023 Kemosabe Records/RCA Records",
          "isDolbyContent": false,
          "songs": []
        },
        {
          "id": "Ql6Agnvw",
          "name": "Love Like This",
          "subtitle": "ZAYN - Love Like This",
          "type": "song",
          "language": "english",
          "playCount": 83020,
          "explicit": false,
          "year": 2023,
          "url": "https://www.jiosaavn.com/song/love-like-this/IQRdcBNeQUQ",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/922/Love-Like-This-English-2023-20230721063956-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/922/Love-Like-This-English-2023-20230721063956-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/922/Love-Like-This-English-2023-20230721063956-500x500.jpg"
            }
          ],
          "releaseDate": "2023-07-21",
          "artistMap": {
            "primaryArtists": [
              {
                "id": "1881894",
                "name": "ZAYN",
                "role": "singer",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/zayn-songs/sG6-OV5nnNU_"
              }
            ],
            "featuredArtists": [],
            "artists": [
              {
                "id": "1825828",
                "name": "Jordan K. Johnson",
                "role": "",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/jordan-k.-johnson-songs/9iT4-qqOwIM_"
              },
              {
                "id": "672854",
                "name": "Stefan Johnson",
                "role": "",
                "image": "https://c.saavncdn.com/613/Sophia-Grace-Rosie-s-Royal-Adventure-English-2014-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/stefan-johnson-songs/J2c,DBR4dso_"
              },
              {
                "id": "1115484",
                "name": "Jonathan Bellion",
                "role": "",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/jonathan-bellion-songs/TKc04IZgCaw_"
              },
              {
                "id": "1887754",
                "name": "Michael Pollack",
                "role": "",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/michael-pollack-songs/kdY6-WgNJ5M_"
              },
              {
                "id": "755911",
                "name": "Zain Javadd Malik",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/zain-javadd-malik-songs/AldrxNHCYaw_"
              },
              {
                "id": "1881894",
                "name": "ZAYN",
                "role": "singer",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/zayn-songs/sG6-OV5nnNU_"
              }
            ]
          },
          "songCount": null,
          "copyrightText": "℗ 2023 Drop Zed Music, LLC, under an exclusive license to Mercury Records / Republic Records, a division of UMG Recordings, Inc.",
          "isDolbyContent": false,
          "songs": []
        },
        {
          "id": "47448795",
          "name": "Rajini The Jailer",
          "subtitle": "",
          "type": "album",
          "language": "hindi",
          "playCount": 0,
          "explicit": false,
          "year": 0,
          "url": "https://www.jiosaavn.com/album/rajini-the-jailer/e6w9GeCLaHU_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/862/Rajini-The-Jailer-Hindi-2023-20230810132955-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/862/Rajini-The-Jailer-Hindi-2023-20230810132955-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/862/Rajini-The-Jailer-Hindi-2023-20230810132955-500x500.jpg"
            }
          ],
          "releaseDate": "2023-08-11",
          "artistMap": {
            "primaryArtists": [],
            "featuredArtists": [],
            "artists": [
              {
                "id": "455663",
                "name": "Anirudh Ravichander",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/anirudh-ravichander-songs/lBzQd8c-xCY_"
              },
              {
                "id": "739770",
                "name": "Raqueeb Alam",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/raqueeb-alam-songs/OubvCu9W,UY_"
              },
              {
                "id": "455663",
                "name": "Anirudh Ravichander",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/anirudh-ravichander-songs/lBzQd8c-xCY_"
              },
              {
                "id": "455663",
                "name": "Anirudh Ravichander",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/anirudh-ravichander-songs/lBzQd8c-xCY_"
              },
              {
                "id": "739770",
                "name": "Raqueeb Alam",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/raqueeb-alam-songs/OubvCu9W,UY_"
              }
            ]
          },
          "songCount": 0,
          "copyrightText": null,
          "isDolbyContent": null,
          "songs": []
        },
        {
          "id": "47149162",
          "name": "Jhat Pat (From \"Love-All\")",
          "subtitle": "",
          "type": "album",
          "language": "hindi",
          "playCount": 0,
          "explicit": false,
          "year": 0,
          "url": "https://www.jiosaavn.com/album/jhat-pat-from-love-all/aDQVMannr3k_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/249/Jhat-Pat-From-Love-All-Hindi-2023-20230731162513-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/249/Jhat-Pat-From-Love-All-Hindi-2023-20230731162513-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/249/Jhat-Pat-From-Love-All-Hindi-2023-20230731162513-500x500.jpg"
            }
          ],
          "releaseDate": "2023-08-11",
          "artistMap": {
            "primaryArtists": [],
            "featuredArtists": [],
            "artists": [
              {
                "id": "455125",
                "name": "Sonu Nigam",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/sonu-nigam-songs/,kuQK6K6u0I_"
              },
              {
                "id": "4451815",
                "name": "Saurabh Vaibhav",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/saurabh-vaibhav-songs/3xpuM78u,8g_"
              },
              {
                "id": "455125",
                "name": "Sonu Nigam",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/sonu-nigam-songs/,kuQK6K6u0I_"
              },
              {
                "id": "4451815",
                "name": "Saurabh Vaibhav",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/saurabh-vaibhav-songs/3xpuM78u,8g_"
              },
              {
                "id": "455125",
                "name": "Sonu Nigam",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/sonu-nigam-songs/,kuQK6K6u0I_"
              },
              {
                "id": "4451815",
                "name": "Saurabh Vaibhav",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/saurabh-vaibhav-songs/3xpuM78u,8g_"
              }
            ]
          },
          "songCount": 0,
          "copyrightText": null,
          "isDolbyContent": null,
          "songs": []
        },
        {
          "id": "cXG8RHdZ",
          "name": "Seven (feat. Latto) (Clean Ver.)",
          "subtitle": "Latto, Jung Kook - Seven (feat. Latto)",
          "type": "song",
          "language": "english",
          "playCount": 294042,
          "explicit": true,
          "year": 2023,
          "url": "https://www.jiosaavn.com/song/seven-feat.-latto-clean-ver./EzAsCSZ4U2k",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/870/Seven-feat-Latto-English-2023-20230714093252-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/870/Seven-feat-Latto-English-2023-20230714093252-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/870/Seven-feat-Latto-English-2023-20230714093252-500x500.jpg"
            }
          ],
          "releaseDate": "2023-07-14",
          "artistMap": {
            "primaryArtists": [
              {
                "id": "1437962",
                "name": "Latto",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/latto-songs/1JYqF,NlmlU_"
              },
              {
                "id": "8711782",
                "name": "Jung Kook",
                "role": "singer",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/jung-kook-songs/H-uZ66aVWjo_"
              }
            ],
            "featuredArtists": [],
            "artists": [
              {
                "id": "777522",
                "name": "Andrew Watt",
                "role": "",
                "image": "https://c.saavncdn.com/848/Wild-Youth-English-2016-20170826004007-150x150_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/andrew-watt-songs/PxEA4NI97bU_"
              },
              {
                "id": "843751",
                "name": "Jon Bellion",
                "role": "",
                "image": "https://c.saavncdn.com/artists/Jon_Bellion_20200218145312_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/jon-bellion-songs/JNMR2pR9reQ_"
              },
              {
                "id": "573597",
                "name": "Henry Walter",
                "role": "",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/henry-walter-songs/nLRg5wWo2lI_"
              },
              {
                "id": "982938",
                "name": "Theron Makiel Thomas",
                "role": "",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/theron-makiel-thomas-songs/pZuNO7-9wn0_"
              },
              {
                "id": "1437962",
                "name": "Latto",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/latto-songs/1JYqF,NlmlU_"
              },
              {
                "id": "8711782",
                "name": "Jung Kook",
                "role": "singer",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/jung-kook-songs/H-uZ66aVWjo_"
              }
            ]
          },
          "songCount": null,
          "copyrightText": "© 2023 BIGHIT MUSIC",
          "isDolbyContent": false,
          "songs": []
        },
        {
          "id": "juLK7G4L",
          "name": "SUPPOSED TO BE LOVED",
          "subtitle": "DJ Khaled, LiL Baby, Future - SUPPOSED TO BE LOVED",
          "type": "song",
          "language": "english",
          "playCount": 1249,
          "explicit": false,
          "year": 2023,
          "url": "https://www.jiosaavn.com/song/supposed-to-be-loved/Gh0nekN3A38",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/871/SUPPOSED-TO-BE-LOVED-English-2023-20230811103658-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/871/SUPPOSED-TO-BE-LOVED-English-2023-20230811103658-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/871/SUPPOSED-TO-BE-LOVED-English-2023-20230811103658-500x500.jpg"
            }
          ],
          "releaseDate": "2023-08-11",
          "artistMap": {
            "primaryArtists": [
              {
                "id": "599255",
                "name": "DJ Khaled",
                "role": "singer",
                "image": "https://c.saavncdn.com/artists/DJ_Khaled_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/dj-khaled-songs/4r3-F646lkk_"
              },
              {
                "id": "1986959",
                "name": "LiL Baby",
                "role": "",
                "image": "https://c.saavncdn.com/artists/LiL_Baby_001_20221110200003_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/lil-baby-songs/kSWQLXOGuoQ_"
              },
              {
                "id": "576342",
                "name": "Future",
                "role": "",
                "image": "https://c.saavncdn.com/artists/Future_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/future-songs/4PQNmhO4qlo_"
              }
            ],
            "featuredArtists": [],
            "artists": [
              {
                "id": "613654",
                "name": "Khaled Khaled",
                "role": "",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/khaled-khaled-songs/SAl1VbRafeQ_"
              },
              {
                "id": "521596",
                "name": "James Ingram",
                "role": "",
                "image": "https://c.saavncdn.com/013/Stand-In-the-Light-English-2009-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/james-ingram-songs/MH3yDn5qcoI_"
              },
              {
                "id": "521597",
                "name": "Quincy Jones",
                "role": "",
                "image": "https://c.saavncdn.com/977/Q-Soul-Bossa-Nostra-2010-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/quincy-jones-songs/G2rPebdVuIk_"
              },
              {
                "id": "3487086",
                "name": "Bryan Lamar Simmons",
                "role": "",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/bryan-lamar-simmons-songs/3Q2WlECAHLE_"
              },
              {
                "id": "766001",
                "name": "Dominique Jones",
                "role": "",
                "image": "https://c.saavncdn.com/366/Back-to-the-Future-English-2019-20190727225347-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/dominique-jones-songs/m8qLriOtHlw_"
              },
              {
                "id": "576345",
                "name": "Nayvadius Wilburn",
                "role": "",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/nayvadius-wilburn-songs/OPpdX6c4YWg_"
              },
              {
                "id": "762239",
                "name": "Symere Woods",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/symere-woods-songs/ZJ-n-a-0-wA_"
              },
              {
                "id": "599255",
                "name": "DJ Khaled",
                "role": "singer",
                "image": "https://c.saavncdn.com/artists/DJ_Khaled_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/dj-khaled-songs/4r3-F646lkk_"
              },
              {
                "id": "1986959",
                "name": "LiL Baby",
                "role": "",
                "image": "https://c.saavncdn.com/artists/LiL_Baby_001_20221110200003_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/lil-baby-songs/kSWQLXOGuoQ_"
              },
              {
                "id": "576342",
                "name": "Future",
                "role": "",
                "image": "https://c.saavncdn.com/artists/Future_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/future-songs/4PQNmhO4qlo_"
              }
            ]
          },
          "songCount": null,
          "copyrightText": "℗ 2023 DJ Khaled Productions, Inc., under exclusive license to Def Jam Recordings, a division of UMG Recordings, Inc.",
          "isDolbyContent": false,
          "songs": []
        },
        {
          "id": "47183929",
          "name": "Atrangi (From \"Richie\")",
          "subtitle": "",
          "type": "album",
          "language": "hindi",
          "playCount": 0,
          "explicit": false,
          "year": 0,
          "url": "https://www.jiosaavn.com/album/atrangi-from-richie/tkbXl9FCldY_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/006/Atrangi-From-Richie-Kannada-2023-20230801192857-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/006/Atrangi-From-Richie-Kannada-2023-20230801192857-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/006/Atrangi-From-Richie-Kannada-2023-20230801192857-500x500.jpg"
            }
          ],
          "releaseDate": "2023-08-03",
          "artistMap": {
            "primaryArtists": [],
            "featuredArtists": [],
            "artists": [
              {
                "id": "10286670",
                "name": "Agastya Santhosh",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/agastya-santhosh-songs/pbO3ZsrOyVY_"
              },
              {
                "id": "457750",
                "name": "Vimal Kashyap",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/vimal-kashyap-songs/mzHqUDUA6Q0_"
              },
              {
                "id": "467309",
                "name": "Palak Muchhal",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/palak-muchhal-songs/9RWBvFQhPxw_"
              },
              {
                "id": "10286670",
                "name": "Agastya Santhosh",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/agastya-santhosh-songs/pbO3ZsrOyVY_"
              },
              {
                "id": "457750",
                "name": "Vimal Kashyap",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/vimal-kashyap-songs/mzHqUDUA6Q0_"
              }
            ]
          },
          "songCount": 0,
          "copyrightText": null,
          "isDolbyContent": null,
          "songs": []
        },
        {
          "id": "47113588",
          "name": "Tu Meri Aashiqui (feat. Neil Nitin Mukesh, Shreya Sharma)",
          "subtitle": "",
          "type": "album",
          "language": "hindi",
          "playCount": 0,
          "explicit": false,
          "year": 0,
          "url": "https://www.jiosaavn.com/album/tu-meri-aashiqui-feat.-neil-nitin-mukesh-shreya-sharma/pqgi-bTT2UI_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/876/Tu-Meri-Aashiqui-feat-Neil-Nitin-Mukesh-Shreya-Sharma-Hindi-2023-20230729001755-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/876/Tu-Meri-Aashiqui-feat-Neil-Nitin-Mukesh-Shreya-Sharma-Hindi-2023-20230729001755-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/876/Tu-Meri-Aashiqui-feat-Neil-Nitin-Mukesh-Shreya-Sharma-Hindi-2023-20230729001755-500x500.jpg"
            }
          ],
          "releaseDate": "2023-08-02",
          "artistMap": {
            "primaryArtists": [],
            "featuredArtists": [],
            "artists": [
              {
                "id": "455922",
                "name": "Rashid Khan",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/rashid-khan-songs/d73YLSIPB2o_"
              },
              {
                "id": "455922",
                "name": "Rashid Khan",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/rashid-khan-songs/d73YLSIPB2o_"
              },
              {
                "id": "455922",
                "name": "Rashid Khan",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/rashid-khan-songs/d73YLSIPB2o_"
              },
              {
                "id": "484852",
                "name": "Ankit Tiwari",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/ankit-tiwari-songs/eHCWDfgAqlg_"
              }
            ]
          },
          "songCount": 0,
          "copyrightText": null,
          "isDolbyContent": null,
          "songs": []
        },
        {
          "id": "M6NWvfM3",
          "name": "Give You Love (feat. Jason Derulo)",
          "subtitle": "Jessica Mauboy - Give You Love (feat. Jason Derulo)",
          "type": "song",
          "language": "english",
          "playCount": 950,
          "explicit": false,
          "year": 2023,
          "url": "https://www.jiosaavn.com/song/give-you-love-feat.-jason-derulo/PV4lZgJWegA",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/792/Give-You-Love-feat-Jason-Derulo-English-2023-20230807194751-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/792/Give-You-Love-feat-Jason-Derulo-English-2023-20230807194751-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/792/Give-You-Love-feat-Jason-Derulo-English-2023-20230807194751-500x500.jpg"
            }
          ],
          "releaseDate": "2023-08-11",
          "artistMap": {
            "primaryArtists": [
              {
                "id": "566006",
                "name": "Jessica Mauboy",
                "role": "singer",
                "image": "https://c.saavncdn.com/581/What-Happened-To-Us-Feat-Stan-Walker-2011-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/jessica-mauboy-songs/gWLOIUIO0MI_"
              }
            ],
            "featuredArtists": [],
            "artists": [
              {
                "id": "873876",
                "name": "Blessing Offor",
                "role": "",
                "image": "https://c.saavncdn.com/896/Roots-English-2015-150x150_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/blessing-offor-songs/5y-g8ga0U-E_"
              },
              {
                "id": "14210019",
                "name": "Shannon Busch",
                "role": "",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/shannon-busch-songs/a1eWvI,hnLo_"
              },
              {
                "id": "1020181",
                "name": "Stephen Mowat",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/stephen-mowat-songs/LQKnbEnV4jY_"
              },
              {
                "id": "566006",
                "name": "Jessica Mauboy",
                "role": "singer",
                "image": "https://c.saavncdn.com/581/What-Happened-To-Us-Feat-Stan-Walker-2011-150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/jessica-mauboy-songs/gWLOIUIO0MI_"
              }
            ]
          },
          "songCount": null,
          "copyrightText": "℗ 2023 Jamally Pty Limited",
          "isDolbyContent": false,
          "songs": []
        },
        {
          "id": "47096387",
          "name": "Jala Jala Hai (From \"King Of Kotha (Hindi)\")",
          "subtitle": "",
          "type": "album",
          "language": "hindi",
          "playCount": 0,
          "explicit": false,
          "year": 0,
          "url": "https://www.jiosaavn.com/album/jala-jala-hai-from-king-of-kotha-hindi/j6yfjhL78zo_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/610/Jala-Jala-Hai-From-King-Of-Kotha-Hindi-Hindi-2023-20230728154659-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/610/Jala-Jala-Hai-From-King-Of-Kotha-Hindi-Hindi-2023-20230728154659-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/610/Jala-Jala-Hai-From-King-Of-Kotha-Hindi-Hindi-2023-20230728154659-500x500.jpg"
            }
          ],
          "releaseDate": "2023-07-28",
          "artistMap": {
            "primaryArtists": [],
            "featuredArtists": [],
            "artists": [
              {
                "id": "546740",
                "name": "Jakes Bejoy",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/jakes-bejoy-songs/7PnUvQMSPmE_"
              },
              {
                "id": "712857",
                "name": "Sahil Solanki",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/sahil-solanki-songs/692YVHyMkNw_"
              },
              {
                "id": "1546796",
                "name": "Shilpa Surroch",
                "role": "music",
                "image": "",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/shilpa-surroch-songs/fCuWeyYkyfA_"
              }
            ]
          },
          "songCount": 0,
          "copyrightText": null,
          "isDolbyContent": null,
          "songs": []
        },
        {
          "id": "I_7qb6ha",
          "name": "One in a Million",
          "subtitle": "Bebe Rexha, David Guetta - One in a Million",
          "type": "song",
          "language": "english",
          "playCount": 21733,
          "explicit": false,
          "year": 2023,
          "url": "https://www.jiosaavn.com/song/one-in-a-million/OTdcQBYGX1I",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/413/One-in-a-Million-English-2023-20230804015601-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/413/One-in-a-Million-English-2023-20230804015601-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/413/One-in-a-Million-English-2023-20230804015601-500x500.jpg"
            }
          ],
          "releaseDate": "2023-08-04",
          "artistMap": {
            "primaryArtists": [
              {
                "id": "677717",
                "name": "Bebe Rexha",
                "role": "singer",
                "image": "https://c.saavncdn.com/artists/Bebe_Rexha_20190905072945_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/bebe-rexha-songs/YmSG6eO6GAM_"
              },
              {
                "id": "568665",
                "name": "David Guetta",
                "role": "",
                "image": "https://c.saavncdn.com/artists/David_Guetta_002_20200921152629_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/david-guetta-songs/AnfLTAoYH1I_"
              }
            ],
            "featuredArtists": [],
            "artists": [
              {
                "id": "677717",
                "name": "Bebe Rexha",
                "role": "singer",
                "image": "https://c.saavncdn.com/artists/Bebe_Rexha_20190905072945_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/bebe-rexha-songs/YmSG6eO6GAM_"
              },
              {
                "id": "568665",
                "name": "David Guetta",
                "role": "",
                "image": "https://c.saavncdn.com/artists/David_Guetta_002_20200921152629_150x150.jpg",
                "type": "artist",
                "url": "https://www.jiosaavn.com/artist/david-guetta-songs/AnfLTAoYH1I_"
              }
            ]
          },
          "songCount": null,
          "copyrightText": "℗ 2023 Warner Records Inc.",
          "isDolbyContent": false,
          "songs": []
        }
      ]
    },
    "trending": {
      "title": "Trending Now",
      "subtitle": "",
      "featuredText": null,
      "data": [
        {
          "id": "802336660",
          "name": "Arijit's Sad Songs",
          "subtitle": "129.4K Followers",
          "type": "playlist",
          "url": "https://www.jiosaavn.com/featured/arijits-sad-songs/8RkefqkCO1huOxiEGmm6lQ__",
          "image": "https://c.saavncdn.com/editorial/logo/ArijitSadSongs_20211117063136.jpg?bch=1688463758",
          "language": "",
          "year": 0,
          "play_count": 0,
          "explicit": false,
          "list_count": 0,
          "list_type": "",
          "list": ""
        },
        {
          "id": "45691079",
          "name": "METRO BOOMIN PRESENTS SPIDER-MAN: ACROSS THE SPIDER-VERSE (SOUNDTRACK FROM AND INSPIRED BY THE MOTION PICTURE)",
          "subtitle": "",
          "type": "album",
          "url": "https://www.jiosaavn.com/album/metro-boomin-presents-spider-man%3a-across-the-spider-verse-soundtrack-from-and-inspired-by-the-motion-picture/X4jW-CUt,V0_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/393/METRO-BOOMIN-PRESENTS-SPIDER-MAN-ACROSS-THE-SPIDER-VERSE-SOUNDTRACK-FROM-AND-INSPIRED-BY-THE-MOTION-PICTURE-English-2023-20230602103409-50x50.jpg?bch=469987"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/393/METRO-BOOMIN-PRESENTS-SPIDER-MAN-ACROSS-THE-SPIDER-VERSE-SOUNDTRACK-FROM-AND-INSPIRED-BY-THE-MOTION-PICTURE-English-2023-20230602103409-150x150.jpg?bch=469987"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/393/METRO-BOOMIN-PRESENTS-SPIDER-MAN-ACROSS-THE-SPIDER-VERSE-SOUNDTRACK-FROM-AND-INSPIRED-BY-THE-MOTION-PICTURE-English-2023-20230602103409-500x500.jpg?bch=469987"
            }
          ],
          "language": "english",
          "year": 0,
          "play_count": 0,
          "explicit": false,
          "list_count": 0,
          "list_type": "",
          "list": ""
        },
        {
          "id": "45378258",
          "name": "FAST X",
          "subtitle": "",
          "type": "album",
          "url": "https://www.jiosaavn.com/album/fast-x/lxvot9mT8u0_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/223/FAST-X-English-2023-20230518053633-50x50.jpg?bch=469987"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/223/FAST-X-English-2023-20230518053633-150x150.jpg?bch=469987"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/223/FAST-X-English-2023-20230518053633-500x500.jpg?bch=469987"
            }
          ],
          "language": "english",
          "year": 0,
          "play_count": 0,
          "explicit": false,
          "list_count": 0,
          "list_type": "",
          "list": ""
        },
        {
          "id": "47439200",
          "name": "Dil Ka Telephone 2.0 (From &quot;Dream Girl 2&quot;)",
          "subtitle": "",
          "type": "album",
          "url": "https://www.jiosaavn.com/album/dil-ka-telephone-2.0-from-dream-girl-2/s0N1ifyNvXc_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/649/Dil-Ka-Telephone-2-0-From-Dream-Girl-2-Hindi-2023-20230810125949-50x50.jpg?bch=469987"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/649/Dil-Ka-Telephone-2-0-From-Dream-Girl-2-Hindi-2023-20230810125949-150x150.jpg?bch=469987"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/649/Dil-Ka-Telephone-2-0-From-Dream-Girl-2-Hindi-2023-20230810125949-500x500.jpg?bch=469987"
            }
          ],
          "language": "hindi",
          "year": 0,
          "play_count": 0,
          "explicit": false,
          "list_count": 0,
          "list_type": "",
          "list": ""
        },
        {
          "id": "47339180",
          "name": "OMG 2",
          "subtitle": "",
          "type": "album",
          "url": "https://www.jiosaavn.com/album/omg-2/t5XBxIwvpms_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/526/OMG-2-Hindi-2023-20230808131515-50x50.jpg?bch=469987"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/526/OMG-2-Hindi-2023-20230808131515-150x150.jpg?bch=469987"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/526/OMG-2-Hindi-2023-20230808131515-500x500.jpg?bch=469987"
            }
          ],
          "language": "hindi",
          "year": 0,
          "play_count": 0,
          "explicit": false,
          "list_count": 0,
          "list_type": "",
          "list": ""
        },
        {
          "id": "294580620",
          "name": "Party All Night!",
          "subtitle": "10.5K Followers",
          "type": "playlist",
          "url": "https://www.jiosaavn.com/featured/party-all-night%21/2CQqp7V7jdluOxiEGmm6lQ__",
          "image": "https://c.saavncdn.com/editorial/logo/PartyAllNight-_20191220104633.jpg?bch=1690934508",
          "language": "",
          "year": 0,
          "play_count": 0,
          "explicit": false,
          "list_count": 0,
          "list_type": "",
          "list": ""
        },
        {
          "id": "47279722",
          "name": "Gadar 2",
          "subtitle": "",
          "type": "album",
          "url": "https://www.jiosaavn.com/album/gadar-2/zKU0zGJl3GY_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/716/Gadar-2-Hindi-2023-20230805095149-50x50.jpg?bch=469987"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/716/Gadar-2-Hindi-2023-20230805095149-150x150.jpg?bch=469987"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/716/Gadar-2-Hindi-2023-20230805095149-500x500.jpg?bch=469987"
            }
          ],
          "language": "hindi",
          "year": 0,
          "play_count": 0,
          "explicit": false,
          "list_count": 0,
          "list_type": "",
          "list": ""
        },
        {
          "id": "43502568",
          "name": "Starboy (Deluxe)",
          "subtitle": "",
          "type": "album",
          "url": "https://www.jiosaavn.com/album/starboy-deluxe/VEmi-MtBuUA_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/851/Starboy-Deluxe-English-2023-20230314103244-50x50.jpg?bch=469987"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/851/Starboy-Deluxe-English-2023-20230314103244-150x150.jpg?bch=469987"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/851/Starboy-Deluxe-English-2023-20230314103244-500x500.jpg?bch=469987"
            }
          ],
          "language": "english",
          "year": 0,
          "play_count": 0,
          "explicit": false,
          "list_count": 0,
          "list_type": "",
          "list": ""
        },
        {
          "id": "45946999",
          "name": "Guli Mata",
          "subtitle": "",
          "type": "album",
          "url": "https://www.jiosaavn.com/album/guli-mata/CZOIcCkgMPM_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/696/Guli-Mata-Hindi-2023-20230714050721-50x50.jpg?bch=469987"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/696/Guli-Mata-Hindi-2023-20230714050721-150x150.jpg?bch=469987"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/696/Guli-Mata-Hindi-2023-20230714050721-500x500.jpg?bch=469987"
            }
          ],
          "language": "hindi",
          "year": 0,
          "play_count": 0,
          "explicit": false,
          "list_count": 0,
          "list_type": "",
          "list": ""
        },
        {
          "id": "151497279",
          "name": "Let's Play - BTS",
          "subtitle": "61.6K Followers",
          "type": "playlist",
          "url": "https://www.jiosaavn.com/featured/lets-play-bts/RtkMxMz4dhCO0eMLZZxqsA__",
          "image": "https://c.saavncdn.com/editorial/Let_sPlayBTS_20230601061847.jpg?bch=1689592682",
          "language": "",
          "year": 0,
          "play_count": 0,
          "explicit": false,
          "list_count": 0,
          "list_type": "",
          "list": ""
        },
        {
          "id": "47304157",
          "name": "Gone Girl",
          "subtitle": "",
          "type": "album",
          "url": "https://www.jiosaavn.com/album/gone-girl/KndqEfO44GU_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/099/Gone-Girl-Hindi-2023-20230806053115-50x50.jpg?bch=469987"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/099/Gone-Girl-Hindi-2023-20230806053115-150x150.jpg?bch=469987"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/099/Gone-Girl-Hindi-2023-20230806053115-500x500.jpg?bch=469987"
            }
          ],
          "language": "hindi",
          "year": 0,
          "play_count": 0,
          "explicit": false,
          "list_count": 0,
          "list_type": "",
          "list": ""
        },
        {
          "id": "84576159",
          "name": "Best of Hip Hop - English",
          "subtitle": "4.5K Followers",
          "type": "playlist",
          "url": "https://www.jiosaavn.com/featured/best-of-hip-hop-english/GCV6qzOys2I_",
          "image": "https://c.saavncdn.com/editorial/BestofHipHopEnglish_20230605102410.jpg?bch=1691483782",
          "language": "",
          "year": 0,
          "play_count": 0,
          "explicit": false,
          "list_count": 0,
          "list_type": "",
          "list": ""
        },
        {
          "id": "945969391",
          "name": "Most Searched Songs - English",
          "subtitle": "13.3K Followers",
          "type": "playlist",
          "url": "https://www.jiosaavn.com/featured/most-searched-songs-english/xUOBWZUG6AgGSw2I1RxdhQ__",
          "image": "https://c.saavncdn.com/editorial/MostSearchedSongsEnglish_20230713064117.jpg?bch=1691406172",
          "language": "",
          "year": 0,
          "play_count": 0,
          "explicit": false,
          "list_count": 0,
          "list_type": "",
          "list": ""
        },
        {
          "id": "43669061",
          "name": "Janiye (from the Netflix Film &quot;Chor Nikal Ke Bhaga&quot;)",
          "subtitle": "",
          "type": "album",
          "url": "https://www.jiosaavn.com/album/janiye-from-the-netflix-film-chor-nikal-ke-bhaga/52AWfXIDEFs_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/041/Janiye-from-the-Netflix-Film-Chor-Nikal-Ke-Bhaga-Hindi-2023-20230708233821-50x50.jpg?bch=469987"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/041/Janiye-from-the-Netflix-Film-Chor-Nikal-Ke-Bhaga-Hindi-2023-20230708233821-150x150.jpg?bch=469987"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/041/Janiye-from-the-Netflix-Film-Chor-Nikal-Ke-Bhaga-Hindi-2023-20230708233821-500x500.jpg?bch=469987"
            }
          ],
          "language": "hindi",
          "year": 0,
          "play_count": 0,
          "explicit": false,
          "list_count": 0,
          "list_type": "",
          "list": ""
        },
        {
          "id": "44924631",
          "name": "THIS YEAR (Blessings) (North African Remix)",
          "subtitle": "",
          "type": "album",
          "url": "https://www.jiosaavn.com/album/this-year-blessings-north-african-remix/2q2QHAYXErc_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/464/THIS-YEAR-Blessings-North-African-Remix-English-2023-20230510141401-50x50.jpg?bch=469987"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/464/THIS-YEAR-Blessings-North-African-Remix-English-2023-20230510141401-150x150.jpg?bch=469987"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/464/THIS-YEAR-Blessings-North-African-Remix-English-2023-20230510141401-500x500.jpg?bch=469987"
            }
          ],
          "language": "english",
          "year": 0,
          "play_count": 0,
          "explicit": false,
          "list_count": 0,
          "list_type": "",
          "list": ""
        },
        {
          "id": "46524576",
          "name": "Tumhe Kitna Pyaar Karte (From &quot;Bawaal&quot;)",
          "subtitle": "",
          "type": "album",
          "url": "https://www.jiosaavn.com/album/tumhe-kitna-pyaar-karte-from-bawaal/x6QmS2wjmw4_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/298/Tumhe-Kitna-Pyaar-Karte-From-Bawaal-Hindi-2023-20230707032926-50x50.jpg?bch=469987"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/298/Tumhe-Kitna-Pyaar-Karte-From-Bawaal-Hindi-2023-20230707032926-150x150.jpg?bch=469987"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/298/Tumhe-Kitna-Pyaar-Karte-From-Bawaal-Hindi-2023-20230707032926-500x500.jpg?bch=469987"
            }
          ],
          "language": "hindi",
          "year": 0,
          "play_count": 0,
          "explicit": false,
          "list_count": 0,
          "list_type": "",
          "list": ""
        },
        {
          "id": "45557446",
          "name": "Ram Siya Ram (From &quot;Adipurush&quot;)",
          "subtitle": "",
          "type": "album",
          "url": "https://www.jiosaavn.com/album/ram-siya-ram-from-adipurush/o21EQ2PGLsk_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/916/Ram-Siya-Ram-From-Adipurush-Hindi-2023-20230530192919-50x50.jpg?bch=469987"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/916/Ram-Siya-Ram-From-Adipurush-Hindi-2023-20230530192919-150x150.jpg?bch=469987"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/916/Ram-Siya-Ram-From-Adipurush-Hindi-2023-20230530192919-500x500.jpg?bch=469987"
            }
          ],
          "language": "hindi",
          "year": 0,
          "play_count": 0,
          "explicit": false,
          "list_count": 0,
          "list_type": "",
          "list": ""
        },
        {
          "id": "44013290",
          "name": "The Beginning: Cupid",
          "subtitle": "",
          "type": "album",
          "url": "https://www.jiosaavn.com/album/the-beginning%3a-cupid/8oz6VEXT0mI_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/479/The-Beginning-Cupid-Korean-2023-20230329115948-50x50.jpg?bch=469987"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/479/The-Beginning-Cupid-Korean-2023-20230329115948-150x150.jpg?bch=469987"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/479/The-Beginning-Cupid-Korean-2023-20230329115948-500x500.jpg?bch=469987"
            }
          ],
          "language": "english",
          "year": 0,
          "play_count": 0,
          "explicit": false,
          "list_count": 0,
          "list_type": "",
          "list": ""
        },
        {
          "id": "46395817",
          "name": "Satyaprem Ki Katha",
          "subtitle": "",
          "type": "album",
          "url": "https://www.jiosaavn.com/album/satyaprem-ki-katha/Av3pEhuDqsY_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/996/Satyaprem-Ki-Katha-Hindi-2023-20230712195402-50x50.jpg?bch=469987"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/996/Satyaprem-Ki-Katha-Hindi-2023-20230712195402-150x150.jpg?bch=469987"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/996/Satyaprem-Ki-Katha-Hindi-2023-20230712195402-500x500.jpg?bch=469987"
            }
          ],
          "language": "hindi",
          "year": 0,
          "play_count": 0,
          "explicit": false,
          "list_count": 0,
          "list_type": "",
          "list": ""
        },
        {
          "id": "43109186",
          "name": "Maan Meri Jaan (Afterlife)",
          "subtitle": "",
          "type": "album",
          "url": "https://www.jiosaavn.com/album/maan-meri-jaan-afterlife/x6CjkgbR9w8_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/184/Maan-Meri-Jaan-Afterlife-English-2023-20230310134919-50x50.jpg?bch=469987"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/184/Maan-Meri-Jaan-Afterlife-English-2023-20230310134919-150x150.jpg?bch=469987"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/184/Maan-Meri-Jaan-Afterlife-English-2023-20230310134919-500x500.jpg?bch=469987"
            }
          ],
          "language": "english",
          "year": 0,
          "play_count": 0,
          "explicit": false,
          "list_count": 0,
          "list_type": "",
          "list": ""
        },
        {
          "id": "45770853",
          "name": "Adipurush",
          "subtitle": "",
          "type": "album",
          "url": "https://www.jiosaavn.com/album/adipurush/YZ03kAOU8ow_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/972/Adipurush-Hindi-2023-20230607184755-50x50.jpg?bch=469987"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/972/Adipurush-Hindi-2023-20230607184755-150x150.jpg?bch=469987"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/972/Adipurush-Hindi-2023-20230607184755-500x500.jpg?bch=469987"
            }
          ],
          "language": "hindi",
          "year": 0,
          "play_count": 0,
          "explicit": false,
          "list_count": 0,
          "list_type": "",
          "list": ""
        },
        {
          "id": "45537304",
          "name": "Dance The Night (From Barbie The Album)",
          "subtitle": "",
          "type": "album",
          "url": "https://www.jiosaavn.com/album/dance-the-night-from-barbie-the-album/,d3fJIbyrjU_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/768/Dance-The-Night-From-Barbie-The-Album-English-2023-20230526002150-50x50.jpg?bch=469987"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/768/Dance-The-Night-From-Barbie-The-Album-English-2023-20230526002150-150x150.jpg?bch=469987"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/768/Dance-The-Night-From-Barbie-The-Album-English-2023-20230526002150-500x500.jpg?bch=469987"
            }
          ],
          "language": "english",
          "year": 0,
          "play_count": 0,
          "explicit": false,
          "list_count": 0,
          "list_type": "",
          "list": ""
        },
        {
          "id": "42549895",
          "name": "Kisi Ka Bhai Kisi Ki Jaan",
          "subtitle": "",
          "type": "album",
          "url": "https://www.jiosaavn.com/album/kisi-ka-bhai-kisi-ki-jaan/MBf5fKyNXlY_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/348/Kisi-Ka-Bhai-Kisi-Ki-Jaan-Hindi-2023-20230427184704-50x50.jpg?bch=469987"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/348/Kisi-Ka-Bhai-Kisi-Ki-Jaan-Hindi-2023-20230427184704-150x150.jpg?bch=469987"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/348/Kisi-Ka-Bhai-Kisi-Ki-Jaan-Hindi-2023-20230427184704-500x500.jpg?bch=469987"
            }
          ],
          "language": "hindi",
          "year": 0,
          "play_count": 0,
          "explicit": false,
          "list_count": 0,
          "list_type": "",
          "list": ""
        },
        {
          "id": "42209980",
          "name": "9.45 P.m",
          "subtitle": "",
          "type": "album",
          "url": "https://www.jiosaavn.com/album/9.45-p.m/UrEH3dnwao0_",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/742/9-45-P-m-English-2023-20230201162346-50x50.jpg?bch=469987"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/742/9-45-P-m-English-2023-20230201162346-150x150.jpg?bch=469987"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/742/9-45-P-m-English-2023-20230201162346-500x500.jpg?bch=469987"
            }
          ],
          "language": "english",
          "year": 0,
          "play_count": 0,
          "explicit": false,
          "list_count": 0,
          "list_type": "",
          "list": ""
        }
      ]
    },
    "playlists": {
      "title": "Editorial Picks",
      "subtitle": "",
      "featuredText": null,
      "data": [
        {
          "id": "87510850",
          "name": "Most Streamed Love Songs - English",
          "subtitle": "14.8K Followers",
          "type": "playlist",
          "image": "https://c.saavncdn.com/editorial/MostStreamedLoveSongsEnglish_20230725095103.jpg",
          "url": "https://www.jiosaavn.com/featured/most-streamed-love-songs-english/wHN0Jq26XqA_",
          "explicit": false,
          "song_count": 30,
          "follower_count": 14824,
          "last_updated": 1691379496,
          "firstname": "JioSaavn",
          "user_id": "phulki_user"
        },
        {
          "id": "80632334",
          "name": "Hum Hindustani",
          "subtitle": "55.6K Followers",
          "type": "playlist",
          "image": "https://c.saavncdn.com/editorial/HumHindustani_20230802055725.jpg",
          "url": "https://www.jiosaavn.com/featured/hum-hindustani/WFpyg06H4I8_",
          "explicit": false,
          "song_count": 36,
          "follower_count": 55564,
          "last_updated": 1691608110,
          "firstname": "JioSaavn",
          "user_id": "phulki_user"
        },
        {
          "id": "89593838",
          "name": "So Fresh Dance Hits",
          "subtitle": "11.7K Followers",
          "type": "playlist",
          "image": "https://c.saavncdn.com/editorial/SoFreshDanceHits_20230811104010.jpg",
          "url": "https://www.jiosaavn.com/featured/so-fresh-dance-hits/rEDxb7swgvE_",
          "explicit": false,
          "song_count": 30,
          "follower_count": 11715,
          "last_updated": 1691725986,
          "firstname": "JioSaavn",
          "user_id": "phulki_user"
        },
        {
          "id": "946682072",
          "name": "Most Searched Songs - Hindi",
          "subtitle": "50.7K Followers",
          "type": "playlist",
          "image": "https://c.saavncdn.com/editorial/MostSearchedSongsHindi_20230807095850.jpg",
          "url": "https://www.jiosaavn.com/featured/most-searched-songs-hindi/csv-SfcHUmHc1EngHtQQ2g__",
          "explicit": false,
          "song_count": 30,
          "follower_count": 50706,
          "last_updated": 1691366362,
          "firstname": "JioSaavn",
          "user_id": "phulki_user"
        },
        {
          "id": "146434735",
          "name": "Latest Latin",
          "subtitle": "17.2K Followers",
          "type": "playlist",
          "image": "https://c.saavncdn.com/editorial/LatestLatin_20230811145153.jpg",
          "url": "https://www.jiosaavn.com/featured/latest-latin/iZf3wDHw2nTuCJW60TJk1Q__",
          "explicit": false,
          "song_count": 30,
          "follower_count": 17186,
          "last_updated": 1691740345,
          "firstname": "JioSaavn",
          "user_id": "phulki_user"
        },
        {
          "id": "903166403",
          "name": "Best Of Romance - Hindi",
          "subtitle": "74.8K Followers",
          "type": "playlist",
          "image": "https://c.saavncdn.com/editorial/BestOfRomanceHindi_20230701031821.jpg",
          "url": "https://www.jiosaavn.com/featured/best-of-romance-hindi/SBKnUgjNeMIwkg5tVhI3fw__",
          "explicit": false,
          "song_count": 50,
          "follower_count": 74787,
          "last_updated": 1688145518,
          "firstname": "JioSaavn",
          "user_id": "phulki_user"
        },
        {
          "id": "902306817",
          "name": "VIRALnation",
          "subtitle": "30.5K Followers",
          "type": "playlist",
          "image": "https://c.saavncdn.com/editorial/VIRALnation_20230802091654.jpg",
          "url": "https://www.jiosaavn.com/featured/viralnation/tfVkYjaAbZJieSJqt9HmOQ__",
          "explicit": false,
          "song_count": 30,
          "follower_count": 30529,
          "last_updated": 1691731724,
          "firstname": "JioSaavn",
          "user_id": "phulki_user"
        },
        {
          "id": "101704478",
          "name": "Jhakaas Remakes",
          "subtitle": "348.1K Followers",
          "type": "playlist",
          "image": "https://c.saavncdn.com/editorial/JhakaasRemakes_20230404064405.jpg",
          "url": "https://www.jiosaavn.com/featured/jhakaas-remakes/7e2LtwVBX6JFo9wdEAzFBA__",
          "explicit": false,
          "song_count": 36,
          "follower_count": 348109,
          "last_updated": 1691708925,
          "firstname": "JioSaavn",
          "user_id": "phulki_user"
        },
        {
          "id": "945969391",
          "name": "Most Searched Songs - English",
          "subtitle": "13.3K Followers",
          "type": "playlist",
          "image": "https://c.saavncdn.com/editorial/MostSearchedSongsEnglish_20230713064117.jpg",
          "url": "https://www.jiosaavn.com/featured/most-searched-songs-english/xUOBWZUG6AgGSw2I1RxdhQ__",
          "explicit": false,
          "song_count": 30,
          "follower_count": 13301,
          "last_updated": 1691379172,
          "firstname": "JioSaavn",
          "user_id": "phulki_user"
        },
        {
          "id": "58057412",
          "name": "Let's Play - Arijit Singh - Hindi",
          "subtitle": "712.2K Followers",
          "type": "playlist",
          "image": "https://c.saavncdn.com/editorial/Let_sPlayArijitSinghHindi_20230809073807.jpg",
          "url": "https://www.jiosaavn.com/featured/lets-play-arijit-singh-hindi/Iz0pi7nkjUE_",
          "explicit": false,
          "song_count": 51,
          "follower_count": 712219,
          "last_updated": 1691577826,
          "firstname": "JioSaavn",
          "user_id": "phulki_user"
        },
        {
          "id": "158225563",
          "name": "Hollywood Movie Hits",
          "subtitle": "3.2K Followers",
          "type": "playlist",
          "image": "https://c.saavncdn.com/editorial/HollywoodMovieHits_20230728094711.jpg",
          "url": "https://www.jiosaavn.com/featured/hollywood-movie-hits/JILX9arsAk8wkg5tVhI3fw__",
          "explicit": false,
          "song_count": 50,
          "follower_count": 3180,
          "last_updated": 1691380197,
          "firstname": "JioSaavn",
          "user_id": "phulki_user"
        },
        {
          "id": "330056058",
          "name": "Desi Rock",
          "subtitle": "22K Followers",
          "type": "playlist",
          "image": "https://c.saavncdn.com/editorial/logo/DesiRockSongs_20210120154545.jpg",
          "url": "https://www.jiosaavn.com/featured/desi-rock/DGl4UBWnRqlFo9wdEAzFBA__",
          "explicit": false,
          "song_count": 23,
          "follower_count": 21990,
          "last_updated": 1691710437,
          "firstname": "JioSaavn",
          "user_id": "phulki_user"
        }
      ]
    },
    "mixes": {
      "title": "Pick Your Mood",
      "subtitle": "",
      "featuredText": null,
      "data": [
        {
          "id": "tagCombMix~zerostate^english|hindi~Bollywood|2010s~hindi",
          "image": "https://c.saavncdn.com/blob/tag_comb_mixes/8c20f592e7251f8c0fd3ff80b0bd8ecb_BollywoodDecade2010s_hindi.jpg",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "url": "https://www.jiosaavn.com/s/mix/bollywood_decade_2010s/asuy3quCOlMo6PRmoLWZhKJFyJ6X0S1JqtkAtAb0XRWEUmu95PMwYuEjtdzcmZqDO,Xgg8ZehlA_",
          "playCount": 0,
          "subtitle": "",
          "name": "Bollywood Decade 2010s",
          "type": "mix",
          "year": 0,
          "explicit": false,
          "firstname": "Saavn",
          "lastname": "Editor"
        },
        {
          "id": "tagCombMix~zerostate^english|hindi~Sufi~hindi",
          "image": "https://c.saavncdn.com/blob/tag_comb_mixes/49b1ad96bba48747167a4ff326395826_SoulfulSufi_hindi.jpg",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "url": "https://www.jiosaavn.com/s/mix/soulful_sufi/asuy3quCOlMo6PRmoLWZhKJFyJ6X0S1JqtkAtAb0XRWIEJLmIileBKCPOJAIFo0W",
          "playCount": 0,
          "subtitle": "",
          "name": "Soulful Sufi",
          "type": "mix",
          "year": 0,
          "explicit": false,
          "firstname": "Saavn",
          "lastname": "Editor"
        },
        {
          "id": "tagCombMix~zerostate^english|hindi~Romantic|Hangout~hindi",
          "image": "https://c.saavncdn.com/blob/tag_comb_mixes/2e33b9e0bef4dfbb06ba6c77ee7fabea_PyaarKeSaathi_hindi.jpg",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "url": "https://www.jiosaavn.com/s/mix/pyaar_ke_saathi/asuy3quCOlMo6PRmoLWZhKJFyJ6X0S1JqtkAtAb0XRWHZg,wXCQGASxJOh2VGevKSXQm1h-KY-3mvuAx5fQx0g__",
          "playCount": 0,
          "subtitle": "",
          "name": "Pyaar Ke Saathi",
          "type": "mix",
          "year": 0,
          "explicit": false,
          "firstname": "Saavn",
          "lastname": "Editor"
        },
        {
          "id": "tagCombMix~zerostate^english|hindi~Duet|Romantic~hindi",
          "image": "https://c.saavncdn.com/blob/tag_comb_mixes/80507f44be87c2a659718bb89532a484_RomanticDuets_hindi.jpg",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "url": "https://www.jiosaavn.com/s/mix/romantic_duets/asuy3quCOlMo6PRmoLWZhKJFyJ6X0S1JqtkAtAb0XRV0f7jD5TfqDdQHMmLH0bcysdebmqentoE_",
          "playCount": 0,
          "subtitle": "",
          "name": "Romantic Duets",
          "type": "mix",
          "year": 0,
          "explicit": false,
          "firstname": "Saavn",
          "lastname": "Editor"
        },
        {
          "id": "tagCombMix~zerostate^english|hindi~Item_Number~hindi",
          "image": "https://c.saavncdn.com/blob/tag_comb_mixes/4a70fd63bd45e2099672e194d39937f1_DhamaalItem_hindi.jpg",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "url": "https://www.jiosaavn.com/s/mix/dhamaal_item/asuy3quCOlMo6PRmoLWZhKJFyJ6X0S1JqtkAtAb0XRX-,O76Anw42wQVA,or8y,Rzf26nWjZvY8_",
          "playCount": 0,
          "subtitle": "",
          "name": "Dhamaal Item",
          "type": "mix",
          "year": 0,
          "explicit": false,
          "firstname": "Saavn",
          "lastname": "Editor"
        },
        {
          "id": "tagCombMix~zerostate^english|hindi~Desi_Hip_Hop~hindi",
          "image": "https://c.saavncdn.com/blob/tag_comb_mixes/10988f39556c9d024e7821604ed6c36a_DesiHipHop_hindi.jpg",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "url": "https://www.jiosaavn.com/s/mix/desi_hip_hop/asuy3quCOlMo6PRmoLWZhKJFyJ6X0S1JqtkAtAb0XRUMZh6F78X8,qmx06Q2LbEQoI84kAgWjRY_",
          "playCount": 0,
          "subtitle": "",
          "name": "Desi Hip Hop",
          "type": "mix",
          "year": 0,
          "explicit": false,
          "firstname": "Saavn",
          "lastname": "Editor"
        },
        {
          "id": "tagCombMix~zerostate^english|hindi~Ghazal~hindi",
          "image": "https://c.saavncdn.com/blob/tag_comb_mixes/08e54189cba0b6b12db7f12cb0d5ad4f_GhazalonKiMehfil_hindi.jpg",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "url": "https://www.jiosaavn.com/s/mix/ghazalon_ki_mehfil/asuy3quCOlMo6PRmoLWZhKJFyJ6X0S1JqtkAtAb0XRUubBFxjOVlMkjO5u2BM01p",
          "playCount": 0,
          "subtitle": "",
          "name": "Ghazalon Ki Mehfil",
          "type": "mix",
          "year": 0,
          "explicit": false,
          "firstname": "Saavn",
          "lastname": "Editor"
        },
        {
          "id": "tagCombMix~zerostate^english|hindi~Bollywood|Unwind~hindi",
          "image": "https://c.saavncdn.com/blob/tag_comb_mixes/ad89d8efba80199aaeef2e8c58598681_BollywoodUnwind_hindi.jpg",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "url": "https://www.jiosaavn.com/s/mix/bollywood_unwind/asuy3quCOlMo6PRmoLWZhKJFyJ6X0S1JqtkAtAb0XRWEUmu95PMwYiwzqov1Rp04cJWnQKFRsK-mvuAx5fQx0g__",
          "playCount": 0,
          "subtitle": "",
          "name": "Bollywood Unwind",
          "type": "mix",
          "year": 0,
          "explicit": false,
          "firstname": "Saavn",
          "lastname": "Editor"
        }
      ]
    },
    "radio": {
      "title": "Radio Stations",
      "subtitle": "",
      "featuredText": null,
      "data": [
        {
          "id": "Drivetime",
          "name": "Drivetime",
          "subtitle": "English Radio",
          "type": "radio_station",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/editorial/WeekendDrive_saavn_radio_20201012143301_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/editorial/WeekendDrive_saavn_radio_20201012143301_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/editorial/WeekendDrive_saavn_radio_20201012143301_500x500.jpg"
            }
          ],
          "url": "https://www.saavn.com/s/radio/english-featured-station/Drivetime",
          "explicit": false,
          "description": "Long Drive on your mind? Hit 'Play'!",
          "featuredStationType": "featured",
          "query": "",
          "color": "#cf47c4",
          "language": "english",
          "stationDisplayText": "Drivetime"
        },
        {
          "id": "Nirgun Bhakti",
          "name": "Nirgun Bhakti",
          "subtitle": "Hindi Radio",
          "type": "radio_station",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/editorial/NirgunBhakti_saavn_radio_20201231131533_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/editorial/NirgunBhakti_saavn_radio_20201231131533_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/editorial/NirgunBhakti_saavn_radio_20201231131533_500x500.jpg"
            }
          ],
          "url": "https://www.saavn.com/s/radio/hindi-featured-station/Nirgun-Bhakti",
          "explicit": false,
          "description": "",
          "featuredStationType": "featured",
          "query": "",
          "color": "#cc00f5",
          "language": "hindi",
          "stationDisplayText": "Nirgun Bhakti"
        },
        {
          "id": "Hindi Musical Covers",
          "name": "Hindi Musical Covers",
          "subtitle": "Hindi Radio",
          "type": "radio_station",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/editorial/MusicalCovers_saavn_radio_20210528093259_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/editorial/MusicalCovers_saavn_radio_20210528093259_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/editorial/MusicalCovers_saavn_radio_20210528093259_500x500.jpg"
            }
          ],
          "url": "https://www.saavn.com/s/radio/hindi-featured-station/Hindi-Musical-Covers",
          "explicit": false,
          "description": "",
          "featuredStationType": "featured",
          "query": "",
          "color": "#05a36c",
          "language": "hindi",
          "stationDisplayText": "Hindi Musical Covers"
        },
        {
          "id": "Unlimited Khushiyan",
          "name": "Unlimited Khushiyan",
          "subtitle": "Hindi Radio",
          "type": "radio_station",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/editorial/HappyTunes_saavn_radio_20211129082209_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/editorial/HappyTunes_saavn_radio_20211129082209_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/editorial/HappyTunes_saavn_radio_20211129082209_500x500.jpg"
            }
          ],
          "url": "https://www.saavn.com/s/radio/hindi-featured-station/Unlimited-Khushiyan",
          "explicit": false,
          "description": "Pump Up The Good Vibes & Let Your Spirits Soar!",
          "featuredStationType": "featured",
          "query": "",
          "color": "#a34202",
          "language": "hindi",
          "stationDisplayText": "Unlimited Khushiyan"
        },
        {
          "id": "Workout",
          "name": "Workout",
          "subtitle": "English Radio",
          "type": "radio_station",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/editorial/Workout_saavn_radio_20201012143447_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/editorial/Workout_saavn_radio_20201012143447_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/editorial/Workout_saavn_radio_20201012143447_500x500.jpg"
            }
          ],
          "url": "https://www.saavn.com/s/radio/english-featured-station/Workout",
          "explicit": false,
          "description": "Grab Your Headphones and Get Moving!",
          "featuredStationType": "featured",
          "query": "",
          "color": "#19ecae",
          "language": "english",
          "stationDisplayText": "Workout"
        },
        {
          "id": "Bollywood Retro 70s-80s",
          "name": "Bollywood Retro 70s-80s",
          "subtitle": "Hindi Radio",
          "type": "radio_station",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/editorial/Retro70sand80s_saavn_radio_20211129082651_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/editorial/Retro70sand80s_saavn_radio_20211129082651_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/editorial/Retro70sand80s_saavn_radio_20211129082651_500x500.jpg"
            }
          ],
          "url": "https://www.saavn.com/s/radio/hindi-featured-station/Bollywood-Retro-70s-80s",
          "explicit": false,
          "description": "",
          "featuredStationType": "featured",
          "query": "",
          "color": "#698a11",
          "language": "hindi",
          "stationDisplayText": "Bollywood Retro 70s-80s"
        },
        {
          "id": "Desi Hip Hop",
          "name": "Desi Hip Hop",
          "subtitle": "Hindi Radio",
          "type": "radio_station",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/editorial/DesiHipHop_saavn_radio_20210331122730_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/editorial/DesiHipHop_saavn_radio_20210331122730_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/editorial/DesiHipHop_saavn_radio_20210331122730_500x500.jpg"
            }
          ],
          "url": "https://www.saavn.com/s/radio/hindi-featured-station/Desi-Hip-Hop",
          "explicit": false,
          "description": "",
          "featuredStationType": "featured",
          "query": "",
          "color": "#0743f5",
          "language": "hindi",
          "stationDisplayText": "Desi Hip Hop"
        },
        {
          "id": "Remix Dhamaal",
          "name": "Remix Dhamaal",
          "subtitle": "Hindi Radio",
          "type": "radio_station",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/editorial/SuperRemixes_saavn_radio_20210528093419_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/editorial/SuperRemixes_saavn_radio_20210528093419_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/editorial/SuperRemixes_saavn_radio_20210528093419_500x500.jpg"
            }
          ],
          "url": "https://www.saavn.com/s/radio/hindi-featured-station/Remix-Dhamaal",
          "explicit": false,
          "description": "",
          "featuredStationType": "featured",
          "query": "",
          "color": "#4d06bf",
          "language": "hindi",
          "stationDisplayText": "Remix Dhamaal"
        },
        {
          "id": "K Pop",
          "name": "K Pop",
          "subtitle": "Korean Radio",
          "type": "radio_station",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/editorial/KPop_saavn_radio_20210507102922_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/editorial/KPop_saavn_radio_20210507102922_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/editorial/KPop_saavn_radio_20210507102922_500x500.jpg"
            }
          ],
          "url": "https://www.saavn.com/s/radio/english-featured-station/K-Pop",
          "explicit": false,
          "description": "",
          "featuredStationType": "featured",
          "query": "",
          "color": "#484a49",
          "language": "english",
          "stationDisplayText": "K Pop"
        },
        {
          "id": "Dukhi Mann",
          "name": "Dukhi Mann",
          "subtitle": "Hindi Radio",
          "type": "radio_station",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/editorial/FeelingSad_saavn_radio_20201120115705_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/editorial/FeelingSad_saavn_radio_20201120115705_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/editorial/FeelingSad_saavn_radio_20201120115705_500x500.jpg"
            }
          ],
          "url": "https://www.saavn.com/s/radio/hindi-featured-station/Dukhi-Mann",
          "explicit": false,
          "description": "",
          "featuredStationType": "featured",
          "query": "",
          "color": "#915656",
          "language": "hindi",
          "stationDisplayText": "Dukhi Mann"
        },
        {
          "id": "Jai Ganesh Deva",
          "name": "Jai Ganesh Deva",
          "subtitle": "Hindi Radio",
          "type": "radio_station",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/editorial/JaiGaneshDeva_saavn_radio_20210331122419_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/editorial/JaiGaneshDeva_saavn_radio_20210331122419_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/editorial/JaiGaneshDeva_saavn_radio_20210331122419_500x500.jpg"
            }
          ],
          "url": "https://www.saavn.com/s/radio/hindi-featured-station/Jai-Ganesh-Deva",
          "explicit": false,
          "description": "",
          "featuredStationType": "featured",
          "query": "",
          "color": "#07b804",
          "language": "hindi",
          "stationDisplayText": "Jai Ganesh Deva"
        },
        {
          "id": "Yeshu Masih",
          "name": "Yeshu Masih",
          "subtitle": "Hindi Radio",
          "type": "radio_station",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/editorial/YeshuMasih_saavn_radio_20211203122614_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/editorial/YeshuMasih_saavn_radio_20211203122614_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/editorial/YeshuMasih_saavn_radio_20211203122614_500x500.jpg"
            }
          ],
          "url": "https://www.saavn.com/s/radio/hindi-featured-station/Yeshu-Masih",
          "explicit": false,
          "description": "",
          "featuredStationType": "featured",
          "query": "",
          "color": "#576362",
          "language": "hindi",
          "stationDisplayText": "Yeshu Masih"
        },
        {
          "id": "Blues",
          "name": "Blues",
          "subtitle": "English Radio",
          "type": "radio_station",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/editorial/Blues_saavn_radio_20210528084631_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/editorial/Blues_saavn_radio_20210528084631_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/editorial/Blues_saavn_radio_20210528084631_500x500.jpg"
            }
          ],
          "url": "https://www.saavn.com/s/radio/english-featured-station/Blues",
          "explicit": false,
          "description": "",
          "featuredStationType": "featured",
          "query": "",
          "color": "#a16a05",
          "language": "english",
          "stationDisplayText": "Blues"
        },
        {
          "id": "90s Nostalgia",
          "name": "90s Nostalgia",
          "subtitle": "Hindi Radio",
          "type": "radio_station",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/editorial/90sNostalgia_saavn_radio_20201012131819_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/editorial/90sNostalgia_saavn_radio_20201012131819_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/editorial/90sNostalgia_saavn_radio_20201012131819_500x500.jpg"
            }
          ],
          "url": "https://www.saavn.com/s/radio/hindi-featured-station/90s-Nostalgia",
          "explicit": false,
          "description": "A Trip down memory lane with Cassette Classics!",
          "featuredStationType": "featured",
          "query": "",
          "color": "#ad7d1c",
          "language": "hindi",
          "stationDisplayText": "90s Nostalgia"
        },
        {
          "id": "Madhur Kirtan",
          "name": "Madhur Kirtan",
          "subtitle": "Hindi Radio",
          "type": "radio_station",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/editorial/MadhurKirtan_saavn_radio_20201215120716_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/editorial/MadhurKirtan_saavn_radio_20201215120716_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/editorial/MadhurKirtan_saavn_radio_20201215120716_500x500.jpg"
            }
          ],
          "url": "https://www.saavn.com/s/radio/hindi-featured-station/Madhur-Kirtan",
          "explicit": false,
          "description": "",
          "featuredStationType": "featured",
          "query": "",
          "color": "#7a7600",
          "language": "hindi",
          "stationDisplayText": "Madhur Kirtan"
        },
        {
          "id": "Hindi Jhankar Beats",
          "name": "Hindi Jhankar Beats",
          "subtitle": "Hindi Radio",
          "type": "radio_station",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/editorial/JhankarBeats_saavn_radio_20201215120219_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/editorial/JhankarBeats_saavn_radio_20201215120219_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/editorial/JhankarBeats_saavn_radio_20201215120219_500x500.jpg"
            }
          ],
          "url": "https://www.saavn.com/s/radio/hindi-featured-station/Hindi-Jhankar-Beats",
          "explicit": false,
          "description": "",
          "featuredStationType": "featured",
          "query": "",
          "color": "#008f81",
          "language": "hindi",
          "stationDisplayText": "Hindi Jhankar Beats"
        },
        {
          "id": "Hip Hop",
          "name": "Hip Hop",
          "subtitle": "English Radio",
          "type": "radio_station",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/editorial/HipHop_saavn_radio_20201012143830_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/editorial/HipHop_saavn_radio_20201012143830_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/editorial/HipHop_saavn_radio_20201012143830_500x500.jpg"
            }
          ],
          "url": "https://www.saavn.com/s/radio/english-featured-station/Hip-Hop",
          "explicit": false,
          "description": "The Hottest destination for Hip Hop Hits!",
          "featuredStationType": "featured",
          "query": "",
          "color": "#c68701",
          "language": "english",
          "stationDisplayText": "Hip Hop"
        },
        {
          "id": "Jai Jinendra",
          "name": "Jai Jinendra",
          "subtitle": "Hindi Radio",
          "type": "radio_station",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/editorial/JaiJinendra_saavn_radio_20220809142934_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/editorial/JaiJinendra_saavn_radio_20220809142934_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/editorial/JaiJinendra_saavn_radio_20220809142934_500x500.jpg"
            }
          ],
          "url": "https://www.saavn.com/s/radio/hindi-featured-station/Jai-Jinendra",
          "explicit": false,
          "description": "",
          "featuredStationType": "featured",
          "query": "",
          "color": "#055db5",
          "language": "hindi",
          "stationDisplayText": "Jai Jinendra"
        },
        {
          "id": "Jai Shiv Shankar",
          "name": "Jai Shiv Shankar",
          "subtitle": "Hindi Radio",
          "type": "radio_station",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/editorial/JaiShivShankar_saavn_radio_20210226154640_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/editorial/JaiShivShankar_saavn_radio_20210226154640_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/editorial/JaiShivShankar_saavn_radio_20210226154640_500x500.jpg"
            }
          ],
          "url": "https://www.saavn.com/s/radio/hindi-featured-station/Jai-Shiv-Shankar",
          "explicit": false,
          "description": "",
          "featuredStationType": "featured",
          "query": "",
          "color": "#737373",
          "language": "hindi",
          "stationDisplayText": "Jai Shiv Shankar"
        },
        {
          "id": "Chill Karo",
          "name": "Chill Karo",
          "subtitle": "Hindi Radio",
          "type": "radio_station",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/editorial/Chill_saavn_radio_20211129080934_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/editorial/Chill_saavn_radio_20211129080934_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/editorial/Chill_saavn_radio_20211129080934_500x500.jpg"
            }
          ],
          "url": "https://www.saavn.com/s/radio/hindi-featured-station/Chill-Karo",
          "explicit": false,
          "description": "Your Destination to Relax and Unwind. Just Chill.",
          "featuredStationType": "featured",
          "query": "",
          "color": "#7f9651",
          "language": "hindi",
          "stationDisplayText": "Chill Karo"
        },
        {
          "id": "Hit Factory",
          "name": "Hit Factory",
          "subtitle": "English Radio",
          "type": "radio_station",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/editorial/HitFactory_saavn_radio_20210226153042_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/editorial/HitFactory_saavn_radio_20210226153042_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/editorial/HitFactory_saavn_radio_20210226153042_500x500.jpg"
            }
          ],
          "url": "https://www.saavn.com/s/radio/english-featured-station/Hit-Factory",
          "explicit": false,
          "description": "",
          "featuredStationType": "featured",
          "query": "",
          "color": "#0290bf",
          "language": "english",
          "stationDisplayText": "Hit Factory"
        },
        {
          "id": "Bhakti Rachna",
          "name": "Bhakti Rachna",
          "subtitle": "Hindi Radio",
          "type": "radio_station",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/editorial/SantVaani_saavn_radio_20210528092948_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/editorial/SantVaani_saavn_radio_20210528092948_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/editorial/SantVaani_saavn_radio_20210528092948_500x500.jpg"
            }
          ],
          "url": "https://www.saavn.com/s/radio/hindi-featured-station/Bhakti-Rachna",
          "explicit": false,
          "description": "",
          "featuredStationType": "featured",
          "query": "",
          "color": "#f25e5e",
          "language": "hindi",
          "stationDisplayText": "Bhakti Rachna"
        },
        {
          "id": "Hindustani Sangeet",
          "name": "Hindustani Sangeet",
          "subtitle": "Hindi Radio",
          "type": "radio_station",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/editorial/HindustaniSangeet_saavn_radio_20210528093123_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/editorial/HindustaniSangeet_saavn_radio_20210528093123_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/editorial/HindustaniSangeet_saavn_radio_20210528093123_500x500.jpg"
            }
          ],
          "url": "https://www.saavn.com/s/radio/hindi-featured-station/Hindustani-Sangeet",
          "explicit": false,
          "description": "",
          "featuredStationType": "featured",
          "query": "",
          "color": "#510487",
          "language": "hindi",
          "stationDisplayText": "Hindustani Sangeet"
        },
        {
          "id": "Sufiyana Safar",
          "name": "Sufiyana Safar",
          "subtitle": "Hindi Radio",
          "type": "radio_station",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/editorial/SufiyanaSafar_saavn_radio_20201125121205_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/editorial/SufiyanaSafar_saavn_radio_20201125121205_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/editorial/SufiyanaSafar_saavn_radio_20201125121205_500x500.jpg"
            }
          ],
          "url": "https://www.saavn.com/s/radio/hindi-featured-station/Sufiyana-Safar",
          "explicit": false,
          "description": "Feel the Spiritual Journey of Sufis, with Sufiyana Safar!",
          "featuredStationType": "featured",
          "query": "",
          "color": "#827453",
          "language": "hindi",
          "stationDisplayText": "Sufiyana Safar"
        },
        {
          "id": "EDM",
          "name": "EDM",
          "subtitle": "English Radio",
          "type": "radio_station",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/editorial/ElectronicMovement_saavn_radio_20201012143356_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/editorial/ElectronicMovement_saavn_radio_20201012143356_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/editorial/ElectronicMovement_saavn_radio_20201012143356_500x500.jpg"
            }
          ],
          "url": "https://www.saavn.com/s/radio/english-featured-station/EDM",
          "explicit": false,
          "description": "",
          "featuredStationType": "featured",
          "query": "",
          "color": "#ea1753",
          "language": "english",
          "stationDisplayText": "EDM"
        },
        {
          "id": "Baal Geet",
          "name": "Baal Geet",
          "subtitle": "Hindi Radio",
          "type": "radio_station",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/editorial/BaalGeet_saavn_radio_20210528092613_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/editorial/BaalGeet_saavn_radio_20210528092613_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/editorial/BaalGeet_saavn_radio_20210528092613_500x500.jpg"
            }
          ],
          "url": "https://www.saavn.com/s/radio/hindi-featured-station/Baal-Geet",
          "explicit": false,
          "description": "",
          "featuredStationType": "featured",
          "query": "",
          "color": "#e86807",
          "language": "hindi",
          "stationDisplayText": "Baal Geet"
        },
        {
          "id": "Mehfil-e-Ghazal",
          "name": "Mehfil-e-Ghazal",
          "subtitle": "Hindi Radio",
          "type": "radio_station",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/editorial/MehfilEGhazal_saavn_radio_20201012131922_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/editorial/MehfilEGhazal_saavn_radio_20201012131922_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/editorial/MehfilEGhazal_saavn_radio_20201012131922_500x500.jpg"
            }
          ],
          "url": "https://www.saavn.com/s/radio/hindi-featured-station/Mehfil-e-Ghazal",
          "explicit": false,
          "description": "From Mehdi Hasan to Jagjit Singh, we've got your Naaz-o-Adaa right here!",
          "featuredStationType": "featured",
          "query": "",
          "color": "#e16a00",
          "language": "hindi",
          "stationDisplayText": "Mehfil-e-Ghazal"
        },
        {
          "id": "Hindi Superhits",
          "name": "Hindi Superhits",
          "subtitle": "Hindi Radio",
          "type": "radio_station",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/editorial/Superhits_saavn_radio_20210226152608_50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/editorial/Superhits_saavn_radio_20210226152608_150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/editorial/Superhits_saavn_radio_20210226152608_500x500.jpg"
            }
          ],
          "url": "https://www.saavn.com/s/radio/hindi-featured-station/Hindi-Superhits",
          "explicit": false,
          "description": "",
          "featuredStationType": "featured",
          "query": "",
          "color": "#cc0c02",
          "language": "hindi",
          "stationDisplayText": "Hindi Superhits"
        }
      ]
    },
    "promo107": {
      "title": "Trending Podcasts",
      "subtitle": "Most heard in India",
      "featuredText": null,
      "data": [
        {
          "id": "71214",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/DHADKANE-MERI-SUN-20230517103017-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/DHADKANE-MERI-SUN-20230517103017-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/DHADKANE-MERI-SUN-20230517103017-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/dhadkane-meri-sun/1/YZxHhNAGfv0_",
          "subtitle": "",
          "name": "DHADKANE MERI SUN",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/DHADKANE-MERI-SUN-20230517103017-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "394604",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/Chanakya-Neeti-Sutra-Sahit-20230809101340-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/Chanakya-Neeti-Sutra-Sahit-20230809101340-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/Chanakya-Neeti-Sutra-Sahit-20230809101340-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/chanakya-neeti-sutra-sahit/1/CMEB,LzNhdA_",
          "subtitle": "",
          "name": "Chanakya Neeti (Sutra Sahit)",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/Chanakya-Neeti-Sutra-Sahit-20230809101340-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "1236",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/Jai-Bajrangi-20210421124459-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/Jai-Bajrangi-20210421124459-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/Jai-Bajrangi-20210421124459-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/jai-bajrangi/1/IVCK,fYNf8E_",
          "subtitle": "",
          "name": "Jai Bajrangi",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/Jai-Bajrangi-20210421124459-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "352727",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/-20230317124005-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/-20230317124005-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/-20230317124005-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/-%e0%a4%b5%e0%a4%bf%e0%a4%b5%e0%a5%87%e0%a4%95%e0%a4%be%e0%a4%a8%e0%a4%82%e0%a4%a6-%e0%a4%95%e0%a5%87-%e0%a4%b5%e0%a4%bf%e0%a4%9a%e0%a4%be%e0%a4%b0-/1/2OaP-vNIFOU_",
          "subtitle": "",
          "name": " विवेकानंद के विचार ",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/-20230317124005-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "315657",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/Yeh-Hain-Chahtein-Author-Vandana-Sharma-20221021065541-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/Yeh-Hain-Chahtein-Author-Vandana-Sharma-20221021065541-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/Yeh-Hain-Chahtein-Author-Vandana-Sharma-20221021065541-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/yeh-hain-chahtein-%7c-%e0%a4%af%e0%a5%87-%e0%a4%b9%e0%a5%88-%e0%a4%9a%e0%a4%be%e0%a4%b9%e0%a4%a4%e0%a5%87%e0%a4%82-%7c-author-vandana-sharma/1/k0qpuv3bAC0_",
          "subtitle": "",
          "name": "Yeh Hain Chahtein | ये है चाहतें | Author- Vandana Sharma",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/Yeh-Hain-Chahtein-Author-Vandana-Sharma-20221021065541-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "183456",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/Bal-Krishan-Leela-20221229195518-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/Bal-Krishan-Leela-20221229195518-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/Bal-Krishan-Leela-20221229195518-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/bal-krishan-leela/1/xW2kT-Clm8A_",
          "subtitle": "",
          "name": "Bal Krishan Leela",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/Bal-Krishan-Leela-20221229195518-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "139418",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/-20210831150529-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/-20210831150529-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/-20210831150529-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/%e0%a4%87%e0%a4%82%e0%a4%a6%e0%a5%81%e0%a4%b0%e0%a5%80%e0%a4%95%e0%a4%b0-%e0%a4%ae%e0%a4%b9%e0%a4%be%e0%a4%b0%e0%a4%be%e0%a4%9c/1/JHUr1DMD8h0_",
          "subtitle": "",
          "name": "इंदुरीकर महाराज",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/-20210831150529-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "76454",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/Sathi-Das-20210715050424-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/Sathi-Das-20210715050424-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/Sathi-Das-20210715050424-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/sathi-das/1/lu9fdg10pGw_",
          "subtitle": "",
          "name": "Sathi Das",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/Sathi-Das-20210715050424-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "63565",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/Shri-Krishna-Amritvani-20211001160841-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/Shri-Krishna-Amritvani-20211001160841-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/Shri-Krishna-Amritvani-20211001160841-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/shri-krishna-amritvani/1/q3nfP8Sr8ZM_",
          "subtitle": "",
          "name": "Shri Krishna Amritvani",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/Shri-Krishna-Amritvani-20211001160841-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "315580",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/Yakshini-Author-Anand-Usha-Borkar-20221020131314-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/Yakshini-Author-Anand-Usha-Borkar-20221020131314-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/Yakshini-Author-Anand-Usha-Borkar-20221020131314-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/yakshini-%7c-%e0%a4%af%e0%a4%95%e0%a5%8d%e0%a4%b7%e0%a4%bf%e0%a4%a3%e0%a5%80-%7c-author-anand-usha-borkar/1/WCMItclbQk8_",
          "subtitle": "",
          "name": "Yakshini | यक्षिणी | Author - Anand Usha Borkar",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/Yakshini-Author-Anand-Usha-Borkar-20221020131314-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "77703",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/SHRIMAD-BHAGWADGITA-20211001215323-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/SHRIMAD-BHAGWADGITA-20211001215323-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/SHRIMAD-BHAGWADGITA-20211001215323-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/shrimad-bhagwadgita/1/WaQepQ3KRz4_",
          "subtitle": "",
          "name": "SHRIMAD BHAGWADGITA",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/SHRIMAD-BHAGWADGITA-20211001215323-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "315656",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/My-CEO-Girl-Author-Simran-Dodke-20221021065420-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/My-CEO-Girl-Author-Simran-Dodke-20221021065420-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/My-CEO-Girl-Author-Simran-Dodke-20221021065420-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/my-ceo-girl-%7c-author-simran-dodke/1/tB63caFkMeM_",
          "subtitle": "",
          "name": "My CEO Girl | Author - Simran Dodke",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/My-CEO-Girl-Author-Simran-Dodke-20221021065420-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "194661",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/-20230102133012-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/-20230102133012-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/-20230102133012-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/-%e0%a4%b2%e0%a4%b5-%e0%a4%b8%e0%a5%8d%e0%a4%9f%e0%a5%8b%e0%a4%b0%e0%a5%80-/1/PGFMH,Db98Q_",
          "subtitle": "",
          "name": " लव स्टोरी ",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/-20230102133012-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "314998",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/Tumne-Kisi-Se-Kabhi-Pyaar-Kiya-Hai-20230531113515-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/Tumne-Kisi-Se-Kabhi-Pyaar-Kiya-Hai-20230531113515-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/Tumne-Kisi-Se-Kabhi-Pyaar-Kiya-Hai-20230531113515-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/tumne-kisi-se-kabhi-pyaar-kiya-hai/1/vb3Z6hVFkKY_",
          "subtitle": "",
          "name": "Tumne Kisi Se Kabhi Pyaar Kiya Hai?",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/Tumne-Kisi-Se-Kabhi-Pyaar-Kiya-Hai-20230531113515-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "41663",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/Akbar-Birbal-Stories-Hindi-Moral-Tales-20221229195245-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/Akbar-Birbal-Stories-Hindi-Moral-Tales-20221229195245-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/Akbar-Birbal-Stories-Hindi-Moral-Tales-20221229195245-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/akbar-birbal-stories-hindi-moral-tales/1/F6CRqp88yw8_",
          "subtitle": "",
          "name": "Akbar Birbal Stories- Hindi Moral Tales",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/Akbar-Birbal-Stories-Hindi-Moral-Tales-20221229195245-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "3087",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/Ramayan-Hindi-20230806123927-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/Ramayan-Hindi-20230806123927-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/Ramayan-Hindi-20230806123927-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/ramayan-hindi/1/VrqL6MVpteI_",
          "subtitle": "",
          "name": "Ramayan Hindi",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/Ramayan-Hindi-20230806123927-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "164110",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/Yatharth-Geeta-Hindi-20211207111212-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/Yatharth-Geeta-Hindi-20211207111212-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/Yatharth-Geeta-Hindi-20211207111212-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/bhagavad-gita-hindi/1/ckb,X9Mcgq4_",
          "subtitle": "",
          "name": "Bhagavad Gita (Hindi)",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/Yatharth-Geeta-Hindi-20211207111212-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "353156",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/Jyotirlinga-20230218073933-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/Jyotirlinga-20230218073933-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/Jyotirlinga-20230218073933-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/%e0%a4%b6%e0%a4%bf%e0%a4%b5-%e0%a4%9c%e0%a5%8d%e0%a4%af%e0%a5%8b%e0%a4%a4%e0%a4%bf%e0%a4%b0%e0%a5%8d%e0%a4%b2%e0%a4%bf%e0%a4%82%e0%a4%97-%e0%a4%95%e0%a5%80-%e0%a4%95%e0%a4%b9%e0%a4%be%e0%a4%a8%e0%a5%80-%7c-shiv-jyotirlinga-12-wonderful-stories-of-12-jyotirlinga/1/y9fDRwwhMpQ_",
          "subtitle": "",
          "name": "शिव ज्योतिर्लिंग की कहानी | Shiv Jyotirlinga (12 Wonderful Stories of 12 Jyotirlinga)",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/Jyotirlinga-20230218073933-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "315926",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/Hanuman-Chalisa-Explanation-20221104123422-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/Hanuman-Chalisa-Explanation-20221104123422-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/Hanuman-Chalisa-Explanation-20221104123422-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/hanuman-chalisa-explanation/1/TkB2L7mGtQI_",
          "subtitle": "",
          "name": "Hanuman Chalisa ( Explanation)",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/Hanuman-Chalisa-Explanation-20221104123422-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "315686",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/Megh-Ek-Shraap-Author-Anand-Usha-Borkar-20221021113301-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/Megh-Ek-Shraap-Author-Anand-Usha-Borkar-20221021113301-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/Megh-Ek-Shraap-Author-Anand-Usha-Borkar-20221021113301-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/megh-ek-shraap-%7c-%e0%a4%ae%e0%a5%87%e0%a4%98-%e0%a4%8f%e0%a4%95-%e0%a4%b6%e0%a5%8d%e0%a4%b0%e0%a4%be%e0%a4%aa-%7c-author-anand-usha-borkar/1/wpv5iUzWgNU_",
          "subtitle": "",
          "name": "Megh - Ek Shraap  | मेघ - एक श्राप | Author - Anand Usha Borkar",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/Megh-Ek-Shraap-Author-Anand-Usha-Borkar-20221021113301-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        }
      ]
    },
    "promo112": {
      "title": "Mid Year Charts 2023",
      "subtitle": "",
      "featuredText": null,
      "data": [
        {
          "id": "1179495885",
          "image": "https://c.saavncdn.com/editorial/Chartbusters2023Hindi_20230705051727.jpg",
          "url": "https://www.jiosaavn.com/featured/chartbusters-2023-hindi/3CCR7D42nHOdFAUQTknEXA__",
          "subtitle": "Himesh Reshammiya, Kamaal Khan, Palak Muchhal, and more",
          "name": "Chartbusters 2023 - Hindi",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": "hindi",
          "position": 1,
          "releaseYear": null
        },
        {
          "id": "1180899296",
          "image": "https://c.saavncdn.com/editorial/Chartbusters2023English_20230710070658.jpg",
          "url": "https://www.jiosaavn.com/featured/-chartbusters-2023-english/OnoB4SznwI1rxMGEuw5nRg__",
          "subtitle": "Libianca, King, Nick Jonas, and more",
          "name": " Chartbusters 2023 - English",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": "english",
          "position": 1,
          "releaseYear": null
        },
        {
          "id": "1180869660",
          "image": "https://c.saavncdn.com/editorial/TopJioTunes2023Hindi_20230710052959.jpg",
          "url": "https://www.jiosaavn.com/featured/top-jiotunes-2023-hindi/8X1glj2KaiZ62UBQeMeB4Q__",
          "subtitle": "Himesh Reshammiya, Kamaal Khan, Palak Muchhal, and more",
          "name": "Top JioTunes 2023 - Hindi",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "1181157615",
          "image": "https://c.saavncdn.com/editorial/HipHopHits2023English_20230711074630.jpg",
          "url": "https://www.jiosaavn.com/featured/-hip-hop-hits-2023-english/lTzyH,rdOCIzQXa1XvOQjA__",
          "subtitle": "21 Savage, Metro Boomin, The Weeknd, and more",
          "name": " Hip Hop Hits 2023 - English",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "1181112876",
          "image": "https://c.saavncdn.com/editorial/HipHopHits2023Hindi_20230711044945.jpg",
          "url": "https://www.jiosaavn.com/featured/hip-hop-hits-2023-hindi/fvk3eQv,ZNZ5b0dfvYvasw__",
          "subtitle": "Emiway Bantai, Badshah, Karan Aujla, and more",
          "name": "Hip Hop Hits 2023 - Hindi",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "1181052520",
          "image": "https://c.saavncdn.com/editorial/DanceHits2023English_20230710091632.jpg",
          "url": "https://www.jiosaavn.com/featured/dance-hits-2023-english/XJdou4n8lPOAIonqf0gmcg__",
          "subtitle": "Miley Cyrus, Fifty Fifty, Raghav, and more",
          "name": "Dance Hits 2023 - English",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "1181189982",
          "image": "https://c.saavncdn.com/editorial/TopJioTunes2023English_20230711101022.jpg",
          "url": "https://www.jiosaavn.com/featured/top-jiotunes-2023-english/ecTJ-UMLwwUrMQGDkCmGvg__",
          "subtitle": "King, Nick Jonas, Ariana Grande, and more",
          "name": "Top JioTunes 2023 - English",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "1181186061",
          "image": "https://c.saavncdn.com/editorial/EDMHits2023English_20230711095034.jpg",
          "url": "https://www.jiosaavn.com/featured/edm-hits-2023-english/vuBll6WoBjWpQbPNOQ1jmw__",
          "subtitle": "Alan Walker, Sasha Alex Sloan, Jvke, and more",
          "name": "EDM Hits 2023 - English",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "1179504143",
          "image": "https://c.saavncdn.com/editorial/RomanticHits2023Hindi_20230705055313.jpg",
          "url": "https://www.jiosaavn.com/featured/romantic-hits-2023-hindi/t7FLT8Wkzn7wwwnaDll-Rw__",
          "subtitle": "Himesh Reshammiya, Kamaal Khan, Palak Muchhal, and more",
          "name": "Romantic Hits 2023 - Hindi",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "1181160458",
          "image": "https://c.saavncdn.com/editorial/KPopHits2023_20230711080045.jpg",
          "url": "https://www.jiosaavn.com/featured/k-pop-hits-2023/gZjQAfOGMJzuS6o99JG3mQ__",
          "subtitle": "Fifty Fifty, Jisoo, and Taeyang",
          "name": "K-Pop Hits 2023",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "1181196711",
          "image": "https://c.saavncdn.com/editorial/RockHits2023English_20230711103223.jpg",
          "url": "https://www.jiosaavn.com/featured/rock-hits-2023-english/Khe-X1,Oj3OrB59Sr2unUQ__",
          "subtitle": "Rina Sawayama, Linkin Park, and Måneskin",
          "name": "Rock Hits 2023 - English",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": "english",
          "position": 10,
          "releaseYear": null
        },
        {
          "id": "1179506683",
          "image": "https://c.saavncdn.com/editorial/IndipopHits2023Hindi_20230705060139.jpg",
          "url": "https://www.jiosaavn.com/featured/indipop-hits-2023-hindi/921l5WVKTdcC0-RsHAbZ7g__",
          "subtitle": "B Praak, Jaani, Javed-Mohsin, and more",
          "name": "Indipop Hits 2023 - Hindi",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "1180917440",
          "image": "https://c.saavncdn.com/editorial/RomanticHits2023English_20230710083651.jpg",
          "url": "https://www.jiosaavn.com/featured/romantic-hits-2023-english/K0,hPMRU9QmWfAFNItf,3Q__",
          "subtitle": "King, Nick Jonas, Ariana Grande, and more",
          "name": "Romantic Hits 2023 - English",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "1181169507",
          "image": "https://c.saavncdn.com/editorial/LatinHits2023_20230711081030.jpg",
          "url": "https://www.jiosaavn.com/featured/latin-hits-2023/nNwt2JmmAJPbdh3Fl54,TQ__",
          "subtitle": "Bizarrap, Shakira, Karol G, and more",
          "name": "Latin Hits 2023",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "1179504962",
          "image": "https://c.saavncdn.com/editorial/DanceHits2023Hindi_20230705055558.jpg",
          "url": "https://www.jiosaavn.com/featured/dance-hits-2023-hindi/rKOZx2tBY81YufGtEEFmDA__",
          "subtitle": "Arijit Singh, Kumaar, Shekhar Ravjiani, and more",
          "name": "Dance Hits 2023 - Hindi",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "1180919694",
          "image": "https://c.saavncdn.com/editorial/PopHits2023English_20230710090519.jpg",
          "url": "https://www.jiosaavn.com/featured/pop-hits-2023-english/mRjCdYJaLz3bB,U5Yp5iAA__",
          "subtitle": "Libianca, Miley Cyrus, and Fifty Fifty",
          "name": "Pop Hits 2023 - English",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        }
      ]
    },
    "promo113": {
      "title": "New Releases Pop - Hindi",
      "subtitle": "",
      "featuredText": null,
      "data": [
        {
          "id": "47168143",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/504/Dard-Hindi-2023-20230807105001-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/504/Dard-Hindi-2023-20230807105001-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/504/Dard-Hindi-2023-20230807105001-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/dard/HO3oC128wQw_",
          "subtitle": "",
          "name": "Dard",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2023
        },
        {
          "id": "47094367",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/875/Baarish-Hai-Jaanam-Hindi-2023-20230728210133-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/875/Baarish-Hai-Jaanam-Hindi-2023-20230728210133-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/875/Baarish-Hai-Jaanam-Hindi-2023-20230728210133-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/baarish-hai-jaanam/,0mT77BLcw0_",
          "subtitle": "",
          "name": "Baarish Hai Jaanam",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2023
        },
        {
          "id": "47304157",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/099/Gone-Girl-Hindi-2023-20230806053115-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/099/Gone-Girl-Hindi-2023-20230806053115-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/099/Gone-Girl-Hindi-2023-20230806053115-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/gone-girl/KndqEfO44GU_",
          "subtitle": "",
          "name": "Gone Girl",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2023
        },
        {
          "id": "46611781",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/311/Kasam-Se-Hindi-2023-20230802122439-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/311/Kasam-Se-Hindi-2023-20230802122439-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/311/Kasam-Se-Hindi-2023-20230802122439-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/kasam-se/DHpUyVpRvkk_",
          "subtitle": "",
          "name": "Kasam Se",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2023
        },
        {
          "id": "47331966",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/910/Saj-Ke-Hindi-2023-20230807154247-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/910/Saj-Ke-Hindi-2023-20230807154247-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/910/Saj-Ke-Hindi-2023-20230807154247-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/saj-ke/wrTRoss0JmQ_",
          "subtitle": "",
          "name": "Saj Ke",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2023
        },
        {
          "id": "46995165",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/421/Tere-Saath-Hindi-2023-20230801170138-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/421/Tere-Saath-Hindi-2023-20230801170138-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/421/Tere-Saath-Hindi-2023-20230801170138-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/tere-saath/bYFp0HbaVN8_",
          "subtitle": "",
          "name": "Tere Saath",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2023
        },
        {
          "id": "47338168",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/091/Diljaniya-2-0-Hindi-2023-20230807165008-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/091/Diljaniya-2-0-Hindi-2023-20230807165008-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/091/Diljaniya-2-0-Hindi-2023-20230807165008-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/diljaniya-2.0/aOEr7iz-1iU_",
          "subtitle": "",
          "name": "Diljaniya 2.0",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2023
        },
        {
          "id": "47107837",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/596/Chandni-Hindi-2023-20230729195544-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/596/Chandni-Hindi-2023-20230729195544-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/596/Chandni-Hindi-2023-20230729195544-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/chandni/aUwgVamSteo_",
          "subtitle": "",
          "name": "Chandni",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2023
        },
        {
          "id": "47033490",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/145/Boondein-Marham-Si-Hindi-2023-20230726032353-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/145/Boondein-Marham-Si-Hindi-2023-20230726032353-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/145/Boondein-Marham-Si-Hindi-2023-20230726032353-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/boondein-marham-si/YuG0JP1KzDI_",
          "subtitle": "",
          "name": "Boondein Marham Si",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2023
        },
        {
          "id": "47339177",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/579/Yeh-Desh-Hindi-2023-20230807171009-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/579/Yeh-Desh-Hindi-2023-20230807171009-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/579/Yeh-Desh-Hindi-2023-20230807171009-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/yeh-desh/Gi-7HiOTS4o_",
          "subtitle": "",
          "name": "Yeh Desh",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2023
        },
        {
          "id": "46962371",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/339/High-Har-Pal-Hindi-2023-20230724133300-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/339/High-Har-Pal-Hindi-2023-20230724133300-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/339/High-Har-Pal-Hindi-2023-20230724133300-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/high-har-pal/MxW55g02LZI_",
          "subtitle": "",
          "name": "High Har Pal",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2023
        },
        {
          "id": "47030367",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/047/Industry2-Hindi-2023-20230726013618-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/047/Industry2-Hindi-2023-20230726013618-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/047/Industry2-Hindi-2023-20230726013618-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/industry2/jS38h,KrvEo_",
          "subtitle": "",
          "name": "Industry2",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2023
        },
        {
          "id": "47150076",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/749/Mujhse-Mil-Hindi-2023-20230731170751-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/749/Mujhse-Mil-Hindi-2023-20230731170751-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/749/Mujhse-Mil-Hindi-2023-20230731170751-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/mujhse-mil/si0wM04SjtQ_",
          "subtitle": "",
          "name": "Mujhse Mil",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2023
        },
        {
          "id": "47453188",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/490/Vande-Mataram-2023-Hindi-2023-20230810202009-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/490/Vande-Mataram-2023-Hindi-2023-20230810202009-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/490/Vande-Mataram-2023-Hindi-2023-20230810202009-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/vande-mataram-2023/JWU4Oso-NrQ_",
          "subtitle": "",
          "name": "Vande Mataram  (2023)",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2023
        },
        {
          "id": "47251809",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/120/Bajao-Original-Series-Soundtrack-Hindi-2023-20230807145007-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/120/Bajao-Original-Series-Soundtrack-Hindi-2023-20230807145007-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/120/Bajao-Original-Series-Soundtrack-Hindi-2023-20230807145007-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/bajao-original-series-soundtrack/g3gjXl2ZbHA_",
          "subtitle": "",
          "name": "Bajao (Original Series Soundtrack)",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2023
        },
        {
          "id": "46811651",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/252/Ishq-Ka-Asar-Hindi-2023-20230718054428-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/252/Ishq-Ka-Asar-Hindi-2023-20230718054428-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/252/Ishq-Ka-Asar-Hindi-2023-20230718054428-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/ishq-ka-asar/PBsKYyL,2T8_",
          "subtitle": "",
          "name": "Ishq Ka Asar",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2023
        },
        {
          "id": "47090283",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/779/Suno-Hindi-2023-20230726121339-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/779/Suno-Hindi-2023-20230726121339-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/779/Suno-Hindi-2023-20230726121339-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/suno/y6xpLN6,0VE_",
          "subtitle": "",
          "name": "Suno",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2023
        },
        {
          "id": "47197027",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/028/Murshida-Hindi-2023-20230802055228-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/028/Murshida-Hindi-2023-20230802055228-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/028/Murshida-Hindi-2023-20230802055228-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/murshida/6HNFq1FtfAQ_",
          "subtitle": "",
          "name": "Murshida",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2023
        },
        {
          "id": "47199276",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/278/Lipstick-Hindi-2023-20230802065443-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/278/Lipstick-Hindi-2023-20230802065443-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/278/Lipstick-Hindi-2023-20230802065443-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/lipstick/ZzhrFvLUmBE_",
          "subtitle": "",
          "name": "Lipstick",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2023
        },
        {
          "id": "46886169",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/965/JALWA-Hindi-2023-20230809022828-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/965/JALWA-Hindi-2023-20230809022828-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/965/JALWA-Hindi-2023-20230809022828-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/jalwa/bdTY9,0c6gs_",
          "subtitle": "",
          "name": "JALWA",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2023
        },
        {
          "id": "46990122",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/947/Kudi-Hindi-2023-20230724193202-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/947/Kudi-Hindi-2023-20230724193202-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/947/Kudi-Hindi-2023-20230724193202-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/kudi/pkkbkPUJUvM_",
          "subtitle": "",
          "name": "Kudi",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2023
        },
        {
          "id": "45664234",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/022/Heeriye-feat-Arijit-Singh-Hindi-2023-20230605195039-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/022/Heeriye-feat-Arijit-Singh-Hindi-2023-20230605195039-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/022/Heeriye-feat-Arijit-Singh-Hindi-2023-20230605195039-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/heeriye-feat.-arijit-singh/ESDOfVN2cxY_",
          "subtitle": "",
          "name": "Heeriye (feat. Arijit Singh)",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2023
        },
        {
          "id": "46921345",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/592/Zohrajabeen-Hindi-2023-20230722053350-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/592/Zohrajabeen-Hindi-2023-20230722053350-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/592/Zohrajabeen-Hindi-2023-20230722053350-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/zohrajabeen/0oS2YwKTXX8_",
          "subtitle": "",
          "name": "Zohrajabeen",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2023
        },
        {
          "id": "46701901",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/080/Dil-Bechara-Hindi-2023-20230714194354-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/080/Dil-Bechara-Hindi-2023-20230714194354-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/080/Dil-Bechara-Hindi-2023-20230714194354-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/dil-bechara/ddt-mZLHrpc_",
          "subtitle": "",
          "name": "Dil Bechara",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2023
        },
        {
          "id": "46551302",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/984/Chorr-De-Hindi-2023-20230717170141-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/984/Chorr-De-Hindi-2023-20230717170141-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/984/Chorr-De-Hindi-2023-20230717170141-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/chorr-de/JWsu8NTa51I_",
          "subtitle": "",
          "name": "Chorr De",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2023
        },
        {
          "id": "46701902",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/240/Love-Stereo-Again-Hindi-2023-20230714194424-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/240/Love-Stereo-Again-Hindi-2023-20230714194424-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/240/Love-Stereo-Again-Hindi-2023-20230714194424-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/love-stereo-again/ZBVqTkmV-Zc_",
          "subtitle": "",
          "name": "Love Stereo Again",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2023
        },
        {
          "id": "46983521",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/213/Baadal-Hindi-2023-20230725195136-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/213/Baadal-Hindi-2023-20230725195136-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/213/Baadal-Hindi-2023-20230725195136-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/baadal/8DrAefAEJIs_",
          "subtitle": "",
          "name": "Baadal",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2023
        },
        {
          "id": "46879782",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/953/Chai-Peete-Hain-Hindi-2023-20230720173932-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/953/Chai-Peete-Hain-Hindi-2023-20230720173932-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/953/Chai-Peete-Hain-Hindi-2023-20230720173932-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/chai-peete-hain/OGEjBwN9L8o_",
          "subtitle": "",
          "name": "Chai Peete Hain",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2023
        },
        {
          "id": "47058547",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/463/Hoodie-Royal-Stag-Packaged-Drinking-Water-Boombox-Hindi-2023-20230804112703-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/463/Hoodie-Royal-Stag-Packaged-Drinking-Water-Boombox-Hindi-2023-20230804112703-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/463/Hoodie-Royal-Stag-Packaged-Drinking-Water-Boombox-Hindi-2023-20230804112703-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/hoodie-royal-stag-packaged-drinking-water-boombox/qu03PYqqWVY_",
          "subtitle": "",
          "name": "Hoodie - Royal Stag Packaged Drinking Water Boombox",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2023
        },
        {
          "id": "46866640",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/804/Bin-Tere-Hindi-2023-20230727033305-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/804/Bin-Tere-Hindi-2023-20230727033305-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/804/Bin-Tere-Hindi-2023-20230727033305-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/bin-tere/2XDUaFOd1bU_",
          "subtitle": "",
          "name": "Bin Tere",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2023
        }
      ]
    },
    "promo114": {
      "title": "Religion & Spirituality Podcasts",
      "subtitle": "Awaken Your Spirituality",
      "featuredText": null,
      "data": [
        {
          "id": "63565",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/Shri-Krishna-Amritvani-20211001160841-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/Shri-Krishna-Amritvani-20211001160841-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/Shri-Krishna-Amritvani-20211001160841-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/shri-krishna-amritvani/1/q3nfP8Sr8ZM_",
          "subtitle": "",
          "name": "Shri Krishna Amritvani",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/Shri-Krishna-Amritvani-20211001160841-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "3087",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/Ramayan-Hindi-20230806123927-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/Ramayan-Hindi-20230806123927-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/Ramayan-Hindi-20230806123927-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/ramayan-hindi/1/VrqL6MVpteI_",
          "subtitle": "",
          "name": "Ramayan Hindi",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/Ramayan-Hindi-20230806123927-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "353156",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/Jyotirlinga-20230218073933-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/Jyotirlinga-20230218073933-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/Jyotirlinga-20230218073933-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/%e0%a4%b6%e0%a4%bf%e0%a4%b5-%e0%a4%9c%e0%a5%8d%e0%a4%af%e0%a5%8b%e0%a4%a4%e0%a4%bf%e0%a4%b0%e0%a5%8d%e0%a4%b2%e0%a4%bf%e0%a4%82%e0%a4%97-%e0%a4%95%e0%a5%80-%e0%a4%95%e0%a4%b9%e0%a4%be%e0%a4%a8%e0%a5%80-%7c-shiv-jyotirlinga-12-wonderful-stories-of-12-jyotirlinga/1/y9fDRwwhMpQ_",
          "subtitle": "",
          "name": "शिव ज्योतिर्लिंग की कहानी | Shiv Jyotirlinga (12 Wonderful Stories of 12 Jyotirlinga)",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/Jyotirlinga-20230218073933-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "315926",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/Hanuman-Chalisa-Explanation-20221104123422-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/Hanuman-Chalisa-Explanation-20221104123422-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/Hanuman-Chalisa-Explanation-20221104123422-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/hanuman-chalisa-explanation/1/TkB2L7mGtQI_",
          "subtitle": "",
          "name": "Hanuman Chalisa ( Explanation)",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/Hanuman-Chalisa-Explanation-20221104123422-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "164110",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/Yatharth-Geeta-Hindi-20211207111212-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/Yatharth-Geeta-Hindi-20211207111212-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/Yatharth-Geeta-Hindi-20211207111212-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/bhagavad-gita-hindi/1/ckb,X9Mcgq4_",
          "subtitle": "",
          "name": "Bhagavad Gita (Hindi)",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/Yatharth-Geeta-Hindi-20211207111212-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "160164",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/-20230103043004-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/-20230103043004-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/-20230103043004-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/-%e0%a4%9a%e0%a4%be%e0%a4%a3%e0%a4%95%e0%a5%8d%e0%a4%af-%e0%a4%a8%e0%a5%80%e0%a4%a4%e0%a4%bf-/1/q4ngBM1b18A_",
          "subtitle": "",
          "name": " चाणक्य नीति ",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/-20230103043004-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "158457",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/Kabir-Ke-Dohe-20211106084949-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/Kabir-Ke-Dohe-20211106084949-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/Kabir-Ke-Dohe-20211106084949-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/kabir-ke-dohe/1/7,i-JMM5Peg_",
          "subtitle": "",
          "name": "Kabir Ke Dohe",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/Kabir-Ke-Dohe-20211106084949-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "77541",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/Shiva-Narrated-by-Jackie-Shroff-20221215130408-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/Shiva-Narrated-by-Jackie-Shroff-20221215130408-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/Shiva-Narrated-by-Jackie-Shroff-20221215130408-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/shiva-narrated-by-jackie-shroff/1/tW54FGJv6Xs_",
          "subtitle": "",
          "name": "Shiva - Narrated by Jackie Shroff",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/Shiva-Narrated-by-Jackie-Shroff-20221215130408-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "1237",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/Sai-Baba-20210421124458-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/Sai-Baba-20210421124458-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/Sai-Baba-20210421124458-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/sai-baba/1/dP4XFIVxJ6I_",
          "subtitle": "",
          "name": "Sai Baba",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/Sai-Baba-20210421124458-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "1236",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/Jai-Bajrangi-20210421124459-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/Jai-Bajrangi-20210421124459-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/Jai-Bajrangi-20210421124459-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/jai-bajrangi/1/IVCK,fYNf8E_",
          "subtitle": "",
          "name": "Jai Bajrangi",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/Jai-Bajrangi-20210421124459-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "77703",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/SHRIMAD-BHAGWADGITA-20211001215323-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/SHRIMAD-BHAGWADGITA-20211001215323-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/SHRIMAD-BHAGWADGITA-20211001215323-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/shrimad-bhagwadgita/1/WaQepQ3KRz4_",
          "subtitle": "",
          "name": "SHRIMAD BHAGWADGITA",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/SHRIMAD-BHAGWADGITA-20211001215323-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "158465",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/Shri-Ganesh-Leela-20221229200130-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/Shri-Ganesh-Leela-20221229200130-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/Shri-Ganesh-Leela-20221229200130-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/shri-ganesh-leela/1/kH5,BUIwsmY_",
          "subtitle": "",
          "name": "Shri Ganesh Leela",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/Shri-Ganesh-Leela-20221229200130-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "364622",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/-20230418094628-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/-20230418094628-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/-20230418094628-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/%e0%a4%b8%e0%a4%a6%e0%a5%8d%e0%a4%97%e0%a5%81%e0%a4%b0%e0%a5%81-%e0%a4%b9%e0%a4%bf%e0%a4%a8%e0%a5%8d%e0%a4%a6%e0%a5%80/1/kxzM-edu588_",
          "subtitle": "",
          "name": "सद्गुरु हिन्दी",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/-20230418094628-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "138229",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/Ram-Ji-Ke-Bhajan-20220416014119-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/Ram-Ji-Ke-Bhajan-20220416014119-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/Ram-Ji-Ke-Bhajan-20220416014119-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/ram-ji-ke-bhajan/1/gFHknkr05Fs_",
          "subtitle": "",
          "name": "Ram Ji Ke Bhajan",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/Ram-Ji-Ke-Bhajan-20220416014119-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "162462",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/The-Stories-of-Mahabharata-20220203233747-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/The-Stories-of-Mahabharata-20220203233747-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/The-Stories-of-Mahabharata-20220203233747-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/the-stories-of-mahabharata/1/-Mqm8HVRKQ0_",
          "subtitle": "",
          "name": "The Stories of Mahabharata",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/The-Stories-of-Mahabharata-20220203233747-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "1238",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/Maa-Durga-20210423140428-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/Maa-Durga-20210423140428-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/Maa-Durga-20210423140428-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/maa-durga/1/iFVJQ-8TAEM_",
          "subtitle": "",
          "name": "Maa Durga",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/Maa-Durga-20210423140428-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "55059",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/Practically-Gita-20210110112924-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/Practically-Gita-20210110112924-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/Practically-Gita-20210110112924-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/practically-gita/1/BzkfQ4hgXvA_",
          "subtitle": "",
          "name": "Practically Gita",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/Practically-Gita-20210110112924-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "183456",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/Bal-Krishan-Leela-20221229195518-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/Bal-Krishan-Leela-20221229195518-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/Bal-Krishan-Leela-20221229195518-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/bal-krishan-leela/1/xW2kT-Clm8A_",
          "subtitle": "",
          "name": "Bal Krishan Leela",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/Bal-Krishan-Leela-20221229195518-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "64326",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/Gita-For-Daily-Living-20221119204226-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/Gita-For-Daily-Living-20221119204226-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/Gita-For-Daily-Living-20221119204226-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/gita-for-daily-living/1/GYPQLrwFOLA_",
          "subtitle": "",
          "name": "Gita For Daily Living",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/Gita-For-Daily-Living-20221119204226-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "44936",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.sop.saavncdn.com/Ram-Leela-20210420120503-500x500.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.sop.saavncdn.com/Ram-Leela-20210420120503-500x500.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.sop.saavncdn.com/Ram-Leela-20210420120503-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/shows/ram-leela/1/YYoK44taVVU_",
          "subtitle": "",
          "name": "Ram Leela",
          "type": "show",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": "https://c.sop.saavncdn.com/Ram-Leela-20210420120503-500x500.jpg",
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        }
      ]
    },
    "promo116": {
      "title": "Top Albums - Hindi",
      "subtitle": "",
      "featuredText": null,
      "data": [
        {
          "id": "1025696",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/410/Dil-To-Pagal-Hai-Hindi-1997-20190329145756-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/410/Dil-To-Pagal-Hai-Hindi-1997-20190329145756-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/410/Dil-To-Pagal-Hai-Hindi-1997-20190329145756-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/dil-to-pagal-hai/MpkyruMLllM_",
          "subtitle": "",
          "name": "Dil To Pagal Hai",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 1997
        },
        {
          "id": "10162109",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/516/Tere-Naam-Hindi-2003-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/516/Tere-Naam-Hindi-2003-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/516/Tere-Naam-Hindi-2003-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/tere-naam/jH9u0VNtHeM_",
          "subtitle": "",
          "name": "Tere Naam",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2003
        },
        {
          "id": "38845390",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/871/Brahmastra-Original-Motion-Picture-Soundtrack-Hindi-2022-20221006155213-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/871/Brahmastra-Original-Motion-Picture-Soundtrack-Hindi-2022-20221006155213-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/871/Brahmastra-Original-Motion-Picture-Soundtrack-Hindi-2022-20221006155213-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/brahmastra-original-motion-picture-soundtrack/xq4v9ZFC9iA_",
          "subtitle": "",
          "name": "Brahmastra (Original Motion Picture Soundtrack)",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2022
        },
        {
          "id": "38682222",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/815/Bhediya-Hindi-2022-20221206124543-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/815/Bhediya-Hindi-2022-20221206124543-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/815/Bhediya-Hindi-2022-20221206124543-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/bhediya/wSM2AOubajk_",
          "subtitle": "",
          "name": "Bhediya",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2022
        },
        {
          "id": "43202857",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/792/Ajab-Prem-Ki-Ghazab-Kahani-Hindi-2009-20230304134249-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/792/Ajab-Prem-Ki-Ghazab-Kahani-Hindi-2009-20230304134249-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/792/Ajab-Prem-Ki-Ghazab-Kahani-Hindi-2009-20230304134249-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/ajab-prem-ki-ghazab-kahani/yMMlwybpO2Y_",
          "subtitle": "",
          "name": "Ajab Prem Ki Ghazab Kahani",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2009
        },
        {
          "id": "43203345",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/515/Taal-Hindi-1999-20230304151256-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/515/Taal-Hindi-1999-20230304151256-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/515/Taal-Hindi-1999-20230304151256-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/taal/NNtePbpDEcI_",
          "subtitle": "",
          "name": "Taal",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 1999
        },
        {
          "id": "14279654",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/367/Kedarnath-Hindi-2019-20190219-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/367/Kedarnath-Hindi-2019-20190219-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/367/Kedarnath-Hindi-2019-20190219-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/kedarnath/0VZPWDP3Hhs_",
          "subtitle": "",
          "name": "Kedarnath",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2019
        },
        {
          "id": "43202862",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/386/Pardes-Hindi-1997-20230304135218-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/386/Pardes-Hindi-1997-20230304135218-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/386/Pardes-Hindi-1997-20230304135218-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/pardes/Zfj7zCl8pUo_",
          "subtitle": "",
          "name": "Pardes",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 1997
        },
        {
          "id": "16188900",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/807/Kabir-Singh-Hindi-2019-20190614075009-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/807/Kabir-Singh-Hindi-2019-20190614075009-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/807/Kabir-Singh-Hindi-2019-20190614075009-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/kabir-singh/kLG-OKbVmvM_",
          "subtitle": "",
          "name": "Kabir Singh",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2019
        },
        {
          "id": "38893739",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/734/Champagne-Talk-Hindi-2022-20221008011951-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/734/Champagne-Talk-Hindi-2022-20221008011951-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/734/Champagne-Talk-Hindi-2022-20221008011951-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/champagne-talk/8O,CIqHgSR0_",
          "subtitle": "",
          "name": "Champagne Talk",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2022
        }
      ]
    },
    "promo118": {
      "title": "Top Albums - English",
      "subtitle": "",
      "featuredText": null,
      "data": [
        {
          "id": "1215569",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/851/FOUR-Deluxe--English-2014-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/851/FOUR-Deluxe--English-2014-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/851/FOUR-Deluxe--English-2014-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/four-deluxe/tbBfBMWGM6E_",
          "subtitle": "",
          "name": "FOUR (Deluxe)",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2014
        },
        {
          "id": "36847947",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/974/RENAISSANCE-English-2022-20220802233341-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/974/RENAISSANCE-English-2022-20220802233341-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/974/RENAISSANCE-English-2022-20220802233341-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/renaissance/GmF9LOxJcoI_",
          "subtitle": "",
          "name": "RENAISSANCE",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2022
        },
        {
          "id": "16380328",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/308/Listen-English-2014-20190607044308-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/308/Listen-English-2014-20190607044308-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/308/Listen-English-2014-20190607044308-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/listen/Bz2CSYFa2Qo_",
          "subtitle": "",
          "name": "Listen",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2014
        },
        {
          "id": "2502208",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/273/Encore-English-2016-20190419221937-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/273/Encore-English-2016-20190419221937-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/273/Encore-English-2016-20190419221937-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/encore/4iGw272FXHY_",
          "subtitle": "",
          "name": "Encore",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2016
        },
        {
          "id": "1759503",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/203/This-Is-Acting-English-2016-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/203/This-Is-Acting-English-2016-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/203/This-Is-Acting-English-2016-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/this-is-acting/WvjkPt-7qiM_",
          "subtitle": "",
          "name": "This Is Acting",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2016
        },
        {
          "id": "30892546",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/935/World-Of-Walker-Unknown-2021-20211126205629-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/935/World-Of-Walker-Unknown-2021-20211126205629-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/935/World-Of-Walker-Unknown-2021-20211126205629-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/world-of-walker/fiEAwejsHmQ_",
          "subtitle": "",
          "name": "World Of Walker",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2021
        },
        {
          "id": "36493120",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/130/Special-English-2022-20220804070818-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/130/Special-English-2022-20220804070818-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/130/Special-English-2022-20220804070818-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/special/eZ1mCKwjnLQ_",
          "subtitle": "",
          "name": "Special",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2022
        },
        {
          "id": "13435951",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/248/Evolve-English-2017-20180716230950-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/248/Evolve-English-2017-20180716230950-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/248/Evolve-English-2017-20180716230950-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/evolve/gvCWqZLfVbs_",
          "subtitle": "",
          "name": "Evolve",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2018
        },
        {
          "id": "27978399",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/852/Planet-Her-English-2021-20210621224521-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/852/Planet-Her-English-2021-20210621224521-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/852/Planet-Her-English-2021-20210621224521-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/planet-her/0dhlIg,ymL4_",
          "subtitle": "",
          "name": "Planet Her",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2021
        },
        {
          "id": "26030221",
          "image": [
            {
              "quality": "50x50",
              "link": "https://c.saavncdn.com/983/Justice-English-2021-20210325102906-50x50.jpg"
            },
            {
              "quality": "150x150",
              "link": "https://c.saavncdn.com/983/Justice-English-2021-20210325102906-150x150.jpg"
            },
            {
              "quality": "500x500",
              "link": "https://c.saavncdn.com/983/Justice-English-2021-20210325102906-500x500.jpg"
            }
          ],
          "url": "https://www.jiosaavn.com/album/justice/7R,IUpSYb,0_",
          "subtitle": "",
          "name": "Justice",
          "type": "album",
          "language": "",
          "list": "",
          "listCount": 0,
          "listType": "",
          "playCount": 0,
          "year": 0,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": 2021
        }
      ]
    },
    "promo176": {
      "title": "Best of",
      "subtitle": "",
      "featuredText": null,
      "data": [
        {
          "id": "78211018",
          "image": "https://c.saavncdn.com/editorial/BestofDanceEnglish_20230207045231.jpg",
          "url": "https://www.jiosaavn.com/featured/best-of-dance-english/l-B,YKTNVVc_",
          "subtitle": "Spice, Dua Lipa, Jason Derulo, and more",
          "name": "Best of Dance -  English",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "158226586",
          "image": "https://c.saavncdn.com/editorial/BestOfLatinPop_20230630135408.jpg",
          "url": "https://www.jiosaavn.com/featured/best-of-latin-pop/zfs3622ON9HfemJ68FuXsA__",
          "subtitle": "Daddy Yankee, Luis Fonsi, J Balvin, and more",
          "name": "Best Of Latin Pop",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "410785269",
          "image": "https://c.saavncdn.com/editorial/KpopBoolgeumWeekendEDM_20230125053051.jpg",
          "url": "https://www.jiosaavn.com/featured/best-of-k-pop/-,-31WVKSbiO0eMLZZxqsA__",
          "subtitle": "BTS, BlackPink, and Lisa",
          "name": "Best of K-Pop",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "84576159",
          "image": "https://c.saavncdn.com/editorial/BestofHipHopEnglish_20230605102410.jpg",
          "url": "https://www.jiosaavn.com/featured/best-of-hip-hop-english/GCV6qzOys2I_",
          "subtitle": "Jack Harlow, Lil Nas X, Masked Wolf, and more",
          "name": "Best of Hip Hop - English",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "303128179",
          "image": "https://c.saavncdn.com/editorial/BestofPopEnglish_20230605101859.jpg",
          "url": "https://www.jiosaavn.com/featured/best-of-pop-english/oqKee-6aXESO0eMLZZxqsA__",
          "subtitle": "Justin Bieber, The Kid Laroi, Harry Styles, and more",
          "name": "Best of Pop - English",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "85159322",
          "image": "https://c.saavncdn.com/editorial/BestofEDMEnglish_20230703104001.jpg",
          "url": "https://www.jiosaavn.com/featured/best-of-edm-english/u4jg0E7Pjt4_",
          "subtitle": "Alan Walker, Major Lazer, and David Guetta",
          "name": "Best of EDM - English",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "158224102",
          "image": "https://c.saavncdn.com/editorial/Decadeof2010sRockHitsEnglish_20230207063755.jpg",
          "url": "https://www.jiosaavn.com/featured/best-of-rock-english/z,LOpxTDaBPc1EngHtQQ2g__",
          "subtitle": "Måneskin, Kaleo, and Imagine Dragons",
          "name": "Best of Rock - English",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "103330340",
          "image": "https://c.saavncdn.com/editorial/BestofMetalEnglish_20230705104623.jpg",
          "url": "https://www.jiosaavn.com/featured/best-of-metal-english/LrEkSAM4-VpuOxiEGmm6lQ__",
          "subtitle": "Linkin Park and Breaking Benjamin",
          "name": "Best of Metal - English",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": "english",
          "position": 8,
          "releaseYear": null
        }
      ]
    },
    "promo185": {
      "title": "Best of 90s",
      "subtitle": "",
      "featuredText": null,
      "data": [
        {
          "id": "3379491",
          "image": "https://c.saavncdn.com/editorial/BestOf90sHindi_20230325080752.jpg",
          "url": "https://www.jiosaavn.com/featured/best-of-90s-hindi/j44dgfQrkXY_",
          "subtitle": "Anuradha Paudwal, Kumar Sanu, Alka Yagnik, and more",
          "name": "Best Of 90s - Hindi",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "84576165",
          "image": "https://c.saavncdn.com/editorial/90sHipHop_20230313063054.jpg",
          "url": "https://www.jiosaavn.com/featured/90s-hip-hop/rH5bbrSh6mU_",
          "subtitle": "2pac, Coolio, and Eminem",
          "name": "90s Hip Hop",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "741026794",
          "image": "https://c.saavncdn.com/editorial/logo/90sRomanceEnglish_20210204191645.jpg",
          "url": "https://www.jiosaavn.com/featured/90s-romance-english/bU7LAkUZSw7ufxkxMEIbIw__",
          "subtitle": "Céline Dion, James Horner, Backstreet Boys, and more",
          "name": "90s Romance - English",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "154546814",
          "image": "https://c.saavncdn.com/editorial/90sRomanceHindi_20230325080736.jpg",
          "url": "https://www.jiosaavn.com/featured/90s-romance-hindi/8eruAVKRO2nufxkxMEIbIw__",
          "subtitle": "Alka Yagnik, Kumar Sanu, Udit Narayan, and more",
          "name": "90s Romance - Hindi",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "158224184",
          "image": "https://c.saavncdn.com/editorial/AllDayShuffle90sHindi_20221202050456.jpg",
          "url": "https://www.jiosaavn.com/featured/best-of-sad-songs%3a-90s/w0I4mmvR4v7ufxkxMEIbIw__",
          "subtitle": "Dominique, K.K., Anuradha Paudwal, and more",
          "name": "Best Of Sad Songs: 90s",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "829470829",
          "image": "https://c.saavncdn.com/editorial/logo/DecadeOfHeroines1990s_20210824153137.jpg",
          "url": "https://www.jiosaavn.com/featured/decade-of-heroines-1990s/eggAXNMLX6GO0eMLZZxqsA__",
          "subtitle": "Alka Yagnik, Udit Narayan, Shankar Mahadevan, and more",
          "name": "Decade Of Heroines - 1990s",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "802132644",
          "image": "https://c.saavncdn.com/editorial/AllDayShuffle90sEnglish_20221107115652.jpg",
          "url": "https://www.jiosaavn.com/featured/best-of-90s-english/dO89Ji-UF1vufxkxMEIbIw__",
          "subtitle": "Backstreet Boys, Céline Dion, James Horner, and more",
          "name": "Best Of 90s - English",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "829466184",
          "image": "https://c.saavncdn.com/editorial/logo/DecadeOfHeroes1990s_20210824153143.jpg",
          "url": "https://www.jiosaavn.com/featured/decade-of-heroes-1990s/M7tTgUcG6efufxkxMEIbIw__",
          "subtitle": "Alka Yagnik, Kumar Sanu, Aamir Khan, and more",
          "name": "Decade Of Heroes - 1990s",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "140767446",
          "image": "https://c.saavncdn.com/editorial/logo/90sPopHits_20210901161257.jpg",
          "url": "https://www.jiosaavn.com/featured/90s-pop-hits/OowFk0Y8YUXfemJ68FuXsA__",
          "subtitle": "Backstreet Boys, Britney Spears, and Savage Garden",
          "name": "90s Pop Hits",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "158225957",
          "image": "https://c.saavncdn.com/editorial/90sMetalHits_20230314050751.jpg",
          "url": "https://www.jiosaavn.com/featured/90s-metal-hits/Y1fWQ23qLhJieSJqt9HmOQ__",
          "subtitle": "Metallica, Megadeth, and Iron Maiden",
          "name": "90s Metal Hits",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "33085749",
          "image": "https://c.saavncdn.com/editorial/logo/90sBoyBands_20210817063254.jpg",
          "url": "https://www.jiosaavn.com/featured/90s-boy-bands/GJYfME1bv,4_",
          "subtitle": "Backstreet Boys, *nsync, and Boyzone",
          "name": "90s Boy Bands",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "83538962",
          "image": "https://c.saavncdn.com/editorial/logo/That90sRock_20210310184208.jpg",
          "url": "https://www.jiosaavn.com/featured/90s-rock-hits/Qpi9A-C6XuA_",
          "subtitle": "Nirvana, Metallica, and Guns N' Roses",
          "name": "90s Rock Hits",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "159470188",
          "image": "https://c.saavncdn.com/editorial/logo/90sDuetsHindiSongs_20210120154331.jpg",
          "url": "https://www.jiosaavn.com/featured/90s-duets-hindi/Icd5-BIRMLpFo9wdEAzFBA__",
          "subtitle": "Kumar Sanu, Lata Mangeshkar, Anand Bakshi, and more",
          "name": "90s Duets - Hindi",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "112817978",
          "image": "https://c.saavncdn.com/editorial/logo/90sHouseParty_20210817063300.jpg",
          "url": "https://www.jiosaavn.com/featured/90s-house-party/Ie4aE52YW91Fo9wdEAzFBA__",
          "subtitle": "Aqua, Vengaboys, and Jennifer Lopez",
          "name": "90s House Party",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "69905756",
          "image": "https://c.saavncdn.com/editorial/TBT90sElectronic_20220914160713.jpg",
          "url": "https://www.jiosaavn.com/featured/90s-electronic/w-YJAma2h94_",
          "subtitle": "Daft Punk, Haddaway, and Eiffel 65",
          "name": "90s Electronic",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        }
      ]
    },
    "promo49": {
      "title": "Monsoon Moments",
      "subtitle": "",
      "featuredText": null,
      "data": [
        {
          "id": "1737128",
          "image": "https://c.saavncdn.com/editorial/BaarishAurPyaar_20230627053430.jpg",
          "url": "https://www.jiosaavn.com/featured/baarish-aur-pyaar/cVWGBIZmLOo_",
          "subtitle": "Arijit Singh, Jubin Nautiyal, Payal Dev, and more",
          "name": "Baarish Aur Pyaar",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": "hindi",
          "position": 1,
          "releaseYear": null
        },
        {
          "id": "107843603",
          "image": "https://c.saavncdn.com/editorial/RainPop_20230627100822.jpg",
          "url": "https://www.jiosaavn.com/featured/rainy-day-pop/nnwppVDhQB4wkg5tVhI3fw__",
          "subtitle": "Rihanna, Taylor Swift, and Ed Sheeran",
          "name": "Rainy Day Pop",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": "english",
          "position": 1,
          "releaseYear": null
        },
        {
          "id": "107946963",
          "image": "https://c.saavncdn.com/editorial/RainyMood_20230712111609.jpg",
          "url": "https://www.jiosaavn.com/featured/smell-of-rain/htfC8oDoh-wwkg5tVhI3fw__",
          "subtitle": "Taylor Swift, Ed Sheeran, and Bruno Mars",
          "name": "Smell of Rain",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": "english",
          "position": 2,
          "releaseYear": null
        },
        {
          "id": "1063585471",
          "image": "https://c.saavncdn.com/editorial/RainingIndipopHindi_20230627053834.jpg",
          "url": "https://www.jiosaavn.com/featured/baarish-aur-indipop/hcDhVgwFSgS8m7pOZz2ziw__",
          "subtitle": "Javed-Mohsin, Shreya Ghoshal, Stebin Ben, and more",
          "name": "Baarish Aur Indipop",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "107844506",
          "image": "https://c.saavncdn.com/editorial/RaindropHipHop_20230628072608.jpg",
          "url": "https://www.jiosaavn.com/featured/raindrop-hip-hop/YX-Nyk1t26TfemJ68FuXsA__",
          "subtitle": "Eminem, Powfu, beabadoobee, and more",
          "name": "Raindrop Hip Hop",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "372014977",
          "image": "https://c.saavncdn.com/editorial/BaarishaurLongdrive_20230627054045.jpg",
          "url": "https://www.jiosaavn.com/featured/baarish-aur-longdrive/eBhB,QnXuddieSJqt9HmOQ__",
          "subtitle": "Shaan, Ustad Sultan Khan, KK, and more",
          "name": "Baarish Aur Longdrive",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "153668826",
          "image": "https://c.saavncdn.com/editorial/RainDanceHindi_20230627053500.jpg",
          "url": "https://www.jiosaavn.com/featured/baarish-aur-dance/NJ4GEZisAeXfemJ68FuXsA__",
          "subtitle": "Shreya Ghoshal, Sonu Nigam, Alka Yagnik, and more",
          "name": "Baarish Aur Dance",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "102635026",
          "image": "https://c.saavncdn.com/editorial/RainyDayRock_20230628072900.jpg",
          "url": "https://www.jiosaavn.com/featured/rainy-day-rock/dJHXbtH,ADnfemJ68FuXsA__",
          "subtitle": "Imagine Dragons, U2, and Red Hot Chili Peppers",
          "name": "Rainy Day Rock",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "107941601",
          "image": "https://c.saavncdn.com/editorial/DardeDilHindi_20230627053550.jpg",
          "url": "https://www.jiosaavn.com/featured/baarish-aur-heartbreak/-zdmruuLEQEGSw2I1RxdhQ__",
          "subtitle": "Atif Aslam, Alka Yagnik, Arijit Singh, and more",
          "name": "Baarish Aur Heartbreak",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "107948909",
          "image": "https://c.saavncdn.com/editorial/RainDance_20230628072758.jpg",
          "url": "https://www.jiosaavn.com/featured/rain-dance-/8aOiYoENAViO0eMLZZxqsA__",
          "subtitle": "Ariana Grande, Lady Gaga, Miley Cyrus, and more",
          "name": "Rain Dance ",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "384232274",
          "image": "https://c.saavncdn.com/editorial/RainyDayClassics_20230629070337.jpg",
          "url": "https://www.jiosaavn.com/featured/rainy-day-classics/eV6dh9Sl5t7ufxkxMEIbIw__",
          "subtitle": "Ray Conniff & The Singers, The Beatles, and The Doors",
          "name": "Rainy Day Classics",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "107977155",
          "image": "https://c.saavncdn.com/editorial/ChaiAurBarsaat_20230627053919.jpg",
          "url": "https://www.jiosaavn.com/featured/baarish-aur-chill/PK9382-FiHTuCJW60TJk1Q__",
          "subtitle": "Amit Trivedi, Amitabh Bhattacharya, Kavita Seth, and more",
          "name": "Baarish Aur Chill",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        }
      ]
    },
    "promo68": {
      "title": "Fresh Hits",
      "subtitle": "",
      "featuredText": null,
      "data": [
        {
          "id": "6689255",
          "image": "https://c.saavncdn.com/editorial/TaazaTunes_20230811083641.jpg",
          "url": "https://www.jiosaavn.com/featured/taaza-tunes/Me5RridRfDk_",
          "subtitle": "Just Updated",
          "name": "Taaza Tunes",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": "hindi",
          "position": 1,
          "releaseYear": null
        },
        {
          "id": "85481065",
          "image": "https://c.saavncdn.com/editorial/FreshTunes_20230811095942.jpg",
          "url": "https://www.jiosaavn.com/featured/fresh-tunes/Ns2UZo9qDvI_",
          "subtitle": "Olivia Rodrigo, V, DJ Khaled, and more",
          "name": "Fresh Tunes",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": "english",
          "position": 1,
          "releaseYear": null
        },
        {
          "id": "81169234",
          "image": "https://c.saavncdn.com/editorial/NowPlayingPop_20230727073040.jpg",
          "url": "https://www.jiosaavn.com/featured/now-playing-pop/IQO11xk800E_",
          "subtitle": "Jung Kook, Latto, ZAYN, and more",
          "name": "Now Playing Pop",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": "english",
          "position": 2,
          "releaseYear": null
        },
        {
          "id": "5148894",
          "image": "https://c.saavncdn.com/editorial/NachLe_20230809043316.jpg",
          "url": "https://www.jiosaavn.com/featured/nach-le/UQjTO6rFZfc_",
          "subtitle": "Aditya Narayan, Mithoon, Udit Narayan, and more",
          "name": "Nach Le",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "81399744",
          "image": "https://c.saavncdn.com/editorial/Heartbeats_20230809083724.jpg",
          "url": "https://www.jiosaavn.com/featured/heartbeats/8j,bJexTYcE_",
          "subtitle": "Armaan Malik, Em Beihold, Stephen Sanchez, and more",
          "name": "Heartbeats",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": "english",
          "position": 4,
          "releaseYear": null
        },
        {
          "id": "158220028",
          "image": "https://c.saavncdn.com/editorial/HindiViralHits_20230705190844.jpg",
          "url": "https://www.jiosaavn.com/featured/hindi-viral-hits/KsNSWjWnbqxFo9wdEAzFBA__",
          "subtitle": "Amitabh Bhattacharya, Arijit Singh, Pritam, and more",
          "name": "Hindi Viral Hits",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "84639340",
          "image": "https://c.saavncdn.com/editorial/JustDance_20230809084858.jpg",
          "url": "https://www.jiosaavn.com/featured/just-dance/ykq09QJIZlU_",
          "subtitle": "Dua Lipa, Madonna, Sam Smith, and more",
          "name": "Just Dance",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "107320028",
          "image": "https://c.saavncdn.com/editorial/HelloKPop_20230811145128.jpg",
          "url": "https://www.jiosaavn.com/featured/hello-k-pop/qmcsGAI9mRFFo9wdEAzFBA__",
          "subtitle": "V, Jung Kook, Latto, and more",
          "name": "Hello K-Pop",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "83421839",
          "image": "https://c.saavncdn.com/editorial/Ishqiyaan_20230707152327.jpg",
          "url": "https://www.jiosaavn.com/featured/ishqiyaan/dyqSeSlJrvI_",
          "subtitle": "Alka Yagnik, Udit Narayan, Arijit Singh, and more",
          "name": "Ishqiyaan",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "173255392",
          "image": "https://c.saavncdn.com/editorial/TheDrop_20230811145047.jpg",
          "url": "https://www.jiosaavn.com/featured/the-drop/NkWn3dHw0Krc1EngHtQQ2g__",
          "subtitle": "Calvin Harris, Sam Smith, Alan Walker, and more",
          "name": "The Drop",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "81304811",
          "image": "https://c.saavncdn.com/editorial/LoveHurts_20230707145923.jpg",
          "url": "https://www.jiosaavn.com/featured/love-hurts/G13cNqMLwqM_",
          "subtitle": "Olivia Rodrigo, David Kushner, and Taylor Swift",
          "name": "Love Hurts",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "156473621",
          "image": "https://c.saavncdn.com/editorial/PopMeinTop_20230728055942.jpg",
          "url": "https://www.jiosaavn.com/featured/pop-mein-top/pDQtHvJRh4IGSw2I1RxdhQ__",
          "subtitle": "B Praak, Jaani, Javed-Mohsin, and more",
          "name": "Pop Mein Top",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "48189087",
          "image": "https://c.saavncdn.com/editorial/EnglishViralHits_20230705155642.jpg",
          "url": "https://www.jiosaavn.com/featured/english-viral-hits/pm49jiq,CNs_",
          "subtitle": "Jain, Daddy Yankee, and Kaleo",
          "name": "English Viral Hits",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "86183802",
          "image": "https://c.saavncdn.com/editorial/RapCypherEnglish_20230804112116.jpg",
          "url": "https://www.jiosaavn.com/featured/rap-cypher-english/nc,LQ5hFF2I_",
          "subtitle": "Travis Scott, Doja Cat, Aqua, and more",
          "name": "Rap Cypher - English",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        }
      ]
    },
    "promo76": {
      "title": "Top Genres & Moods",
      "subtitle": "",
      "featuredText": null,
      "data": [
        {
          "id": "158223987",
          "image": "https://c.saavncdn.com/editorial/ChillMaaro_20230701031848.jpg",
          "url": "https://www.jiosaavn.com/featured/chill-maaro/CZTTBas4j8RieSJqt9HmOQ__",
          "subtitle": "Amit Trivedi, Jasleen Royal, Alyssa Mendonsa, and more",
          "name": "Chill Maaro",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "77076833",
          "image": "https://c.saavncdn.com/editorial/logo/TimelessGhazals_20211215062059.jpg",
          "url": "https://www.jiosaavn.com/featured/timeless-ghazals/Kdb2rW52fiE_",
          "subtitle": "Jagjit Singh, Chitra Singh, and Pankaj Udhas",
          "name": "Timeless Ghazals",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "156515525",
          "image": "https://c.saavncdn.com/editorial/TeenPopLove_20230613111248.jpg",
          "url": "https://www.jiosaavn.com/featured/teen-pop-love/rRMWBHmMhODuCJW60TJk1Q__",
          "subtitle": "Alec Benjamin, Stephen Sanchez, and Jaymes Young",
          "name": "Teen Pop Love",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "158767012",
          "image": "https://c.saavncdn.com/editorial/NewInMetal_20230303111932.jpg",
          "url": "https://www.jiosaavn.com/featured/new-in-metal/Nf3OsreD7kTc1EngHtQQ2g__",
          "subtitle": "Cannibal Corpse, Kataklysm, and Dying Fetus",
          "name": "New In Metal",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": "english",
          "position": 4,
          "releaseYear": null
        },
        {
          "id": "330056058",
          "image": "https://c.saavncdn.com/editorial/logo/DesiRockSongs_20210120154545.jpg",
          "url": "https://www.jiosaavn.com/featured/desi-rock/DGl4UBWnRqlFo9wdEAzFBA__",
          "subtitle": "Ram Sampath , Farhan Akhtar, Shankar Ehsaan Loy, and more",
          "name": "Desi Rock",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "163362514",
          "image": "https://c.saavncdn.com/editorial/logo/AbsoluteClassicRock_20210405183403.jpg",
          "url": "https://www.jiosaavn.com/featured/absolute-classic-rock/bfVvvAV,lf3ufxkxMEIbIw__",
          "subtitle": "Led Zeppelin, Eagles, and Lynyrd Skynyrd",
          "name": "Absolute Classic Rock",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "804316688",
          "image": "https://c.saavncdn.com/editorial/HindiEDM_20220624153714.jpg",
          "url": "https://www.jiosaavn.com/featured/hindi-edm/OYy8Eam-WpZFo9wdEAzFBA__",
          "subtitle": "Kanishk Seth, Kavita Seth, Diplo, and more",
          "name": "Hindi EDM",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "1021029897",
          "image": "https://c.saavncdn.com/editorial/logo/DesiR_B_20220110160141.jpg",
          "url": "https://www.jiosaavn.com/featured/desi-rb/1rsxc6d-ReGTb7czG7lKZg__",
          "subtitle": "King, Badshah, Payal Dev, and more",
          "name": "Desi R&amp;B",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "903166403",
          "image": "https://c.saavncdn.com/editorial/BestOfRomanceHindi_20230701031821.jpg",
          "url": "https://www.jiosaavn.com/featured/best-of-romance-hindi/SBKnUgjNeMIwkg5tVhI3fw__",
          "subtitle": "Arijit Singh, Sachin-Jigar, Amitabh Bhattacharya, and more",
          "name": "Best Of Romance - Hindi",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "61470767",
          "image": "https://c.saavncdn.com/editorial/AllNewCountry_20230609151906.jpg",
          "url": "https://www.jiosaavn.com/featured/all-new-country/WnfLufQpDCg_",
          "subtitle": "Carrie Underwood, Maroon 5, and Kelsea Ballerini",
          "name": "All New Country",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "158224188",
          "image": "https://c.saavncdn.com/editorial/logo/HipHopParty_20201020060123.jpg",
          "url": "https://www.jiosaavn.com/featured/hip-hop-party/w0I4mmvR4v5Fo9wdEAzFBA__",
          "subtitle": "DIVINE, Emiway Bantai, KSHMR, and more",
          "name": "Hip Hop Party",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "87000197",
          "image": "https://c.saavncdn.com/editorial/EssentialSoftPop_20230714135100.jpg",
          "url": "https://www.jiosaavn.com/featured/essential-soft-pop/vw3rxyOhUs8_",
          "subtitle": "Billie Eilish, Libianca, and Ed Sheeran",
          "name": "Essential Soft Pop",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "84902960",
          "image": "https://c.saavncdn.com/editorial/TreadmillPop_20230721135817.jpg",
          "url": "https://www.jiosaavn.com/featured/treadmill-pop/SF9VDfc5vr4_",
          "subtitle": "Britney Spears, Will.I.Am, Jung Kook, and more",
          "name": "Treadmill Pop",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "81852466",
          "image": "https://c.saavncdn.com/editorial/FreshRnB_20230712112504.jpg",
          "url": "https://www.jiosaavn.com/featured/fresh-rnb/kwJkRwJz89k_",
          "subtitle": "Jennie, Lily Rose Depp, The Weeknd, and more",
          "name": "Fresh RnB",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        }
      ]
    },
    "promo90": {
      "title": "All Time Favourites - English",
      "subtitle": "",
      "featuredText": null,
      "data": [
        {
          "id": "1064986202",
          "image": "https://c.saavncdn.com/editorial/Chartbusters2022English_20221207065930.jpg",
          "url": "https://www.jiosaavn.com/featured/chartbusters-2022-english/RDAcxj7fTgOHzIkTq9r3nw__",
          "subtitle": "Harry Styles, Badshah, J Balvin, and more",
          "name": "Chartbusters 2022 - English",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": "english",
          "position": 1,
          "releaseYear": null
        },
        {
          "id": "799202096",
          "image": "https://c.saavncdn.com/editorial/logo/2020ChartbustersEnglish_20210831061257.jpg",
          "url": "https://www.jiosaavn.com/featured/chartbusters-2020-english/bZk60CotZnHfemJ68FuXsA__",
          "subtitle": "Dua Lipa, Justin Bieber, and BTS",
          "name": "Chartbusters 2020 - English",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "81134817",
          "image": "https://c.saavncdn.com/editorial/logo/ShakiraShakira_20210121183715.jpg",
          "url": "https://www.jiosaavn.com/featured/lets-play-shakira/KPGGpMI5Lgo_",
          "subtitle": "Shakira, Freshlyground, Kyla-Rose Smith, and more",
          "name": "Let's Play - Shakira",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "151497279",
          "image": "https://c.saavncdn.com/editorial/Let_sPlayBTS_20230601061847.jpg",
          "url": "https://www.jiosaavn.com/featured/lets-play-bts/RtkMxMz4dhCO0eMLZZxqsA__",
          "subtitle": "BTS",
          "name": "Let's Play - BTS",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "732261797",
          "image": "https://c.saavncdn.com/editorial/2015Chartbusters_20220317195932.jpg",
          "url": "https://www.jiosaavn.com/featured/chartbusters-2015-english/tsC3rjA,7RNieSJqt9HmOQ__",
          "subtitle": "Justin Bieber, Ellie Goulding, and Wiz Khalifa",
          "name": "Chartbusters 2015 - English",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "829429344",
          "image": "https://c.saavncdn.com/editorial/AllDayShuffle2000sEnglish_20221107121612.jpg",
          "url": "https://www.jiosaavn.com/featured/best-of-2000s-english/qI-c,cU4ujfufxkxMEIbIw__",
          "subtitle": "Justin Timberlake, Shakira, and The Black Eyed Peas",
          "name": "Best Of  2000s - English",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        },
        {
          "id": "824507215",
          "image": "https://c.saavncdn.com/editorial/AllDayShuffle80sEnglish_20221107121605.jpg",
          "url": "https://www.jiosaavn.com/featured/best-of-80s-english/o585,v3hDQbuCJW60TJk1Q__",
          "subtitle": "Michael Jackson and Survivor",
          "name": "Best Of  80s - English",
          "type": "playlist",
          "language": null,
          "list": null,
          "listCount": null,
          "listType": null,
          "playCount": null,
          "year": null,
          "explicit": false,
          "squareImage": null,
          "editorialLanguage": null,
          "position": null,
          "releaseYear": null
        }
      ]
    }
  }
}
```

+++

|           **Query** **Parameter**           |                                                                                                    **Description**                                                                                                    |              **Required**               |
| :-----------------------------------------: | :-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------: | :-------------------------------------: |
| [!badge variant="contrast" text="language"] | one or more comma separated languages</br>`hindi`, `english`, `punjabi`, `tamil`, `telugu`, `marathi`,`gujarati`, `bengali`, `kannada`, `bhojpuri`, `malayalam`, `urdu`, `haryanvi`, `rajasthani`, `odia`, `assamese` | [!badge variant="primary" text="False"] |
