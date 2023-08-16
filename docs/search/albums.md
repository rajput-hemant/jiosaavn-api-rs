---
order: 600
icon: stack
---

# Search Albums

Search Albums by query.

!!! Note
<https://jiosaavn.shuttleapp.rs> is only meant to demo the API and has rate-limiting enabled to minimise bandwidth consumption.
It is recommended to deploy your own instance for personal use.
!!!

+++ Request

HTTP

```bash
https://jiosaavn.shuttleapp.rs/search/albums?query=rockstar
```

cURL

```bash
curl -X GET 'https://jiosaavn.shuttleapp.rs/search/albums?query=rockstar' \
 -H 'content-type: application/json'
```

+++ Response

```json
{
  "status": "Success",
  "message": "✅ Search results for albums fetched successfully!",
  "data": {
    "total": 594,
    "start": 1,
    "results": [
      {
        "id": "1045274",
        "name": "Rockstar",
        "subtitle": "A.R. Rahman",
        "type": "album",
        "language": "hindi",
        "playCount": 0,
        "explicit": false,
        "year": 2011,
        "url": "https://www.jiosaavn.com/album/rockstar/C3Br8V0qKrc_",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/408/Rockstar-Hindi-2011-20221212023139-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/408/Rockstar-Hindi-2011-20221212023139-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/408/Rockstar-Hindi-2011-20221212023139-500x500.jpg"
          }
        ],
        "releaseDate": null,
        "artistMap": {
          "primaryArtists": [
            {
              "id": "456269",
              "name": "A.R. Rahman",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/a.r.-rahman-songs/HhFyPLvlKN0_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "456269",
              "name": "A.R. Rahman",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/a.r.-rahman-songs/HhFyPLvlKN0_"
            },
            {
              "id": "455124",
              "name": "Mohit Chauhan",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/mohit-chauhan-songs/YNIh2poJKsY_"
            },
            {
              "id": "455517",
              "name": "Shalini",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/shalini-songs/BhpgnNNljZk_"
            },
            {
              "id": "464748",
              "name": "Tanvi Shah",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/tanvi-shah-songs/BGMZ-vi32vk_"
            },
            {
              "id": "456517",
              "name": "Suvi Suresh",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/suvi-suresh-songs/472H97lhCFo_"
            },
            {
              "id": "458945",
              "name": "Viviane Chaix",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/viviane-chaix-songs/viEuKhVaDPI_"
            },
            {
              "id": "484358",
              "name": "Alma Ferovic",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/alma-ferovic-songs/shoIbZgo9H8_"
            },
            {
              "id": "459632",
              "name": "Ranbir Kapoor",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/ranbir-kapoor-songs/nqAfNmNB5II_"
            },
            {
              "id": "455926",
              "name": "Javed Ali",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/javed-ali-songs/iJXgfejIuJQ_"
            },
            {
              "id": "456269",
              "name": "A.R. Rahman",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/a.r.-rahman-songs/HhFyPLvlKN0_"
            },
            {
              "id": "456051",
              "name": "Clinton Cerejo",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/clinton-cerejo-songs/R1cIAeJHuXw_"
            },
            {
              "id": "455158",
              "name": "Kavita Krishnamurthy",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/kavita-krishnamurthy-songs/jqEqWhvcxbM_"
            },
            {
              "id": "456200",
              "name": "Suzanne D&#039;Mello",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/suzanne-dmello-songs/g8an8nf,m90_"
            },
            {
              "id": "455351",
              "name": "Sapna Awasthi",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sapna-awasthi-songs/IMViaeonnxA_"
            },
            {
              "id": "455491",
              "name": "Harshdeep Kaur",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/harshdeep-kaur-songs/5stC-lsOFTo_"
            },
            {
              "id": "455240",
              "name": "Karthik",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/karthik-songs/KTA5BIxJOFM_"
            }
          ]
        },
        "songCount": 15,
        "copyrightText": null,
        "isDolbyContent": null,
        "songs": []
      },
      {
        "id": "22994802",
        "name": "Rockstar Dhillon",
        "subtitle": "Srmn",
        "type": "album",
        "language": "punjabi",
        "playCount": 0,
        "explicit": false,
        "year": 2020,
        "url": "https://www.jiosaavn.com/album/rockstar-dhillon/3oKbaDql6wE_",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/069/Rockstar-Dhillon-Punjabi-2020-20201011081610-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/069/Rockstar-Dhillon-Punjabi-2020-20201011081610-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/069/Rockstar-Dhillon-Punjabi-2020-20201011081610-500x500.jpg"
          }
        ],
        "releaseDate": null,
        "artistMap": {
          "primaryArtists": [
            {
              "id": "8387784",
              "name": "Srmn",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/srmn-songs/lulHoirEIHY_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "8838391",
              "name": "Srmn Goraya",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/srmn-goraya-songs/2vGxQbIeoAU_"
            },
            {
              "id": "9920798",
              "name": "Ap Dhillon Dhillon",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/ap-dhillon-dhillon-songs/kaSo9,y--Jk_"
            },
            {
              "id": "8353125",
              "name": "Gurinder Gill",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/gurinder-gill-songs/C1tu1qPdK7o_"
            },
            {
              "id": "8387784",
              "name": "Srmn",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/srmn-songs/lulHoirEIHY_"
            }
          ]
        },
        "songCount": 1,
        "copyrightText": null,
        "isDolbyContent": null,
        "songs": []
      },
      {
        "id": "26954476",
        "name": "Rockstar",
        "subtitle": "Dridaxe",
        "type": "album",
        "language": "telugu",
        "playCount": 0,
        "explicit": true,
        "year": 2021,
        "url": "https://www.jiosaavn.com/album/rockstar/rF5fu77yTes_",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/382/Rockstar-Telugu-2021-20210503221958-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/382/Rockstar-Telugu-2021-20210503221958-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/382/Rockstar-Telugu-2021-20210503221958-500x500.jpg"
          }
        ],
        "releaseDate": null,
        "artistMap": {
          "primaryArtists": [
            {
              "id": "8765587",
              "name": "Dridaxe",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/dridaxe-songs/Vdmo-clMUBo_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "8765587",
              "name": "Dridaxe",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/dridaxe-songs/Vdmo-clMUBo_"
            },
            {
              "id": "10469781",
              "name": "Sujeeth Hunny",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/sujeeth-hunny-songs/T3V3-cBneNU_"
            }
          ]
        },
        "songCount": 1,
        "copyrightText": null,
        "isDolbyContent": null,
        "songs": []
      },
      {
        "id": "11917965",
        "name": "Dj Rockstar",
        "subtitle": "Rakesh Barot",
        "type": "album",
        "language": "gujarati",
        "playCount": 0,
        "explicit": false,
        "year": 2017,
        "url": "https://www.jiosaavn.com/album/dj-rockstar/shZUCE-3QzM_",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/105/Dj-Rockstar-Gujarati-2017-20171208-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/105/Dj-Rockstar-Gujarati-2017-20171208-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/105/Dj-Rockstar-Gujarati-2017-20171208-500x500.jpg"
          }
        ],
        "releaseDate": null,
        "artistMap": {
          "primaryArtists": [
            {
              "id": "496020",
              "name": "Rakesh Barot",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/rakesh-barot-songs/TF8s6gMduRs_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "495743",
              "name": "Ajay Vagheshwari",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/ajay-vagheshwari-songs/XM8G9tgRVss_"
            },
            {
              "id": "496020",
              "name": "Rakesh Barot",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/rakesh-barot-songs/TF8s6gMduRs_"
            }
          ]
        },
        "songCount": 19,
        "copyrightText": null,
        "isDolbyContent": null,
        "songs": []
      },
      {
        "id": "14663875",
        "name": "Rockstar",
        "subtitle": "gg",
        "type": "album",
        "language": "punjabi",
        "playCount": 0,
        "explicit": false,
        "year": 2018,
        "url": "https://www.jiosaavn.com/album/rockstar/jjZn0Ds7jzY_",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/388/Rockstar-Punjabi-2018-20181217200540-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/388/Rockstar-Punjabi-2018-20181217200540-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/388/Rockstar-Punjabi-2018-20181217200540-500x500.jpg"
          }
        ],
        "releaseDate": null,
        "artistMap": {
          "primaryArtists": [
            {
              "id": "911968",
              "name": "gg",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/gg-songs/vj0PUsIjFxE_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "5103373",
              "name": "Juggy Gill",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/juggy-gill-songs/qSZY9ROcOyU_"
            },
            {
              "id": "911968",
              "name": "gg",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/gg-songs/vj0PUsIjFxE_"
            }
          ]
        },
        "songCount": 1,
        "copyrightText": null,
        "isDolbyContent": null,
        "songs": []
      },
      {
        "id": "1775270",
        "name": "Desi Rockstar 2",
        "subtitle": "Gippy Grewal",
        "type": "album",
        "language": "punjabi",
        "playCount": 0,
        "explicit": false,
        "year": 2016,
        "url": "https://www.jiosaavn.com/album/desi-rockstar-2/MoqFpfxfPUY_",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/577/Desi-Rockstar-2-Punjabi-2016-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/577/Desi-Rockstar-2-Punjabi-2016-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/577/Desi-Rockstar-2-Punjabi-2016-500x500.jpg"
          }
        ],
        "releaseDate": null,
        "artistMap": {
          "primaryArtists": [
            {
              "id": "457737",
              "name": "Gippy Grewal",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/gippy-grewal-songs/jI,K6Lc,w9s_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "490636",
              "name": "JSL Singh",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/jsl-singh-songs/qYGIev2Sm,8_"
            },
            {
              "id": "474469",
              "name": "Jatinder Shah",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/jatinder-shah-songs/bL27zPf3PAQ_"
            },
            {
              "id": "694459",
              "name": "Dj Flow",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/dj-flow-songs/2YYjjCMgHK0_"
            },
            {
              "id": "461962",
              "name": "Aman Hayer",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/aman-hayer-songs/Koa,sxdu,ng_"
            },
            {
              "id": "457737",
              "name": "Gippy Grewal",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/gippy-grewal-songs/jI,K6Lc,w9s_"
            },
            {
              "id": "456863",
              "name": "Badshah",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/badshah-songs/d4OwAaEcnD0_"
            },
            {
              "id": "490616",
              "name": "Tarannum Mallik Jain",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/tarannum-mallik-jain-songs/tK-tU836-MA_"
            },
            {
              "id": "742683",
              "name": "Fateh",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/fateh-songs/Kd6138z42UI_"
            },
            {
              "id": "456104",
              "name": "Bohemia",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/bohemia-songs/tNeurkanAf0_"
            }
          ]
        },
        "songCount": 11,
        "copyrightText": null,
        "isDolbyContent": null,
        "songs": []
      },
      {
        "id": "45456153",
        "name": "Rockstar - Single",
        "subtitle": "Atif Aslam",
        "type": "album",
        "language": "arabic",
        "playCount": 0,
        "explicit": false,
        "year": 2023,
        "url": "https://www.jiosaavn.com/album/rockstar-single/Y6P-HUelugM_",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/683/Rockstar-Single-Arabic-2023-20230522174951-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/683/Rockstar-Single-Arabic-2023-20230522174951-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/683/Rockstar-Single-Arabic-2023-20230522174951-500x500.jpg"
          }
        ],
        "releaseDate": null,
        "artistMap": {
          "primaryArtists": [
            {
              "id": "459633",
              "name": "Atif Aslam",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/atif-aslam-songs/MXn8bhT308U_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "455090",
              "name": "Traditional",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/traditional-songs/1W8p9iMeXm4_"
            },
            {
              "id": "459633",
              "name": "Atif Aslam",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/atif-aslam-songs/MXn8bhT308U_"
            }
          ]
        },
        "songCount": 1,
        "copyrightText": null,
        "isDolbyContent": null,
        "songs": []
      },
      {
        "id": "17832614",
        "name": "I&#039;m A Rockstar (feat. Tony Kakkar)",
        "subtitle": "Neha Kakkar",
        "type": "album",
        "language": "hindi",
        "playCount": 0,
        "explicit": false,
        "year": 2013,
        "url": "https://www.jiosaavn.com/album/im-a-rockstar-feat.-tony-kakkar/xySF0zr,Tfg_",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/593/I-m-A-Rockstar-feat-Tony-Kakkar--Hindi-2013-20191104213230-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/593/I-m-A-Rockstar-feat-Tony-Kakkar--Hindi-2013-20191104213230-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/593/I-m-A-Rockstar-feat-Tony-Kakkar--Hindi-2013-20191104213230-500x500.jpg"
          }
        ],
        "releaseDate": null,
        "artistMap": {
          "primaryArtists": [
            {
              "id": "464932",
              "name": "Neha Kakkar",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/neha-kakkar-songs/EkEBV7JAU-I_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "455917",
              "name": "Tony Kakkar",
              "role": "music",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/tony-kakkar-songs/97L-IGLIwvA_"
            },
            {
              "id": "464932",
              "name": "Neha Kakkar",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/neha-kakkar-songs/EkEBV7JAU-I_"
            }
          ]
        },
        "songCount": 1,
        "copyrightText": null,
        "isDolbyContent": null,
        "songs": []
      },
      {
        "id": "22557642",
        "name": "Rockstar",
        "subtitle": "Kunda",
        "type": "album",
        "language": "arabic",
        "playCount": 0,
        "explicit": false,
        "year": 2020,
        "url": "https://www.jiosaavn.com/album/rockstar/Rr,qOKvaAX4_",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/454/Rockstar-Arabic-2020-20200918043256-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/454/Rockstar-Arabic-2020-20200918043256-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/454/Rockstar-Arabic-2020-20200918043256-500x500.jpg"
          }
        ],
        "releaseDate": null,
        "artistMap": {
          "primaryArtists": [
            {
              "id": "1748396",
              "name": "Kunda",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/kunda-songs/NpksaTyAZAc_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "1748396",
              "name": "Kunda",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/kunda-songs/NpksaTyAZAc_"
            }
          ]
        },
        "songCount": 1,
        "copyrightText": null,
        "isDolbyContent": null,
        "songs": []
      },
      {
        "id": "29748508",
        "name": "Rockstar",
        "subtitle": "Damage D, Hanikarak, EterNAL Musick",
        "type": "album",
        "language": "hindi",
        "playCount": 0,
        "explicit": true,
        "year": 2021,
        "url": "https://www.jiosaavn.com/album/rockstar/0Z1dkNTUw-k_",
        "image": [
          {
            "quality": "50x50",
            "link": "https://c.saavncdn.com/746/Rockstar-Hindi-2021-20210918135120-50x50.jpg"
          },
          {
            "quality": "150x150",
            "link": "https://c.saavncdn.com/746/Rockstar-Hindi-2021-20210918135120-150x150.jpg"
          },
          {
            "quality": "500x500",
            "link": "https://c.saavncdn.com/746/Rockstar-Hindi-2021-20210918135120-500x500.jpg"
          }
        ],
        "releaseDate": null,
        "artistMap": {
          "primaryArtists": [
            {
              "id": "7747503",
              "name": "Damage D",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/damage-d-songs/AOnAmeGVtK0_"
            },
            {
              "id": "7400754",
              "name": "Hanikarak",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/hanikarak-songs/Cb47OSONC7Y_"
            },
            {
              "id": "10056249",
              "name": "EterNAL Musick",
              "role": "primary_artists",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/eternal-musick-songs/NnVGB9vtiG8_"
            }
          ],
          "featuredArtists": [],
          "artists": [
            {
              "id": "7747503",
              "name": "Damage D",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/damage-d-songs/AOnAmeGVtK0_"
            },
            {
              "id": "7400754",
              "name": "Hanikarak",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/hanikarak-songs/Cb47OSONC7Y_"
            },
            {
              "id": "10056249",
              "name": "EterNAL Musick",
              "role": "singers",
              "image": "",
              "type": "artist",
              "url": "https://www.jiosaavn.com/artist/eternal-musick-songs/NnVGB9vtiG8_"
            }
          ]
        },
        "songCount": 1,
        "copyrightText": null,
        "isDolbyContent": null,
        "songs": []
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
