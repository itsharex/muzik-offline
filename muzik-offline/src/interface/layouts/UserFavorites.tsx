import { artist1, artist2, artist3, artist4, artist5 } from "@assets/index";
import { SquareTitleBox, GeneralContextMenu } from "@components/index";
import { artistDetails, mouse_coOrds, contextMenuEnum } from "types";
import { useState } from "react";
import "@styles/layouts/UserFavorites.scss";

const artists: artistDetails[] = [
    {
        key: 0,
        cover: artist1,
        artist_name: "artist 1",
        favourited: true
    },
    {
        key: 1,
        cover: artist2,
        artist_name: "artist 1",
        favourited: false
    },
    {
        key: 2,
        cover: artist3,
        artist_name: "artist 1",
        favourited: true
    },
    {
        key: 3,
        cover: artist4,
        artist_name: "artist 1",
        favourited: true
    },
    {
        key: 4,
        cover: artist5,
        artist_name: "artist 1",
        favourited: true
    },
    {
        key: 5,
        cover: artist4,
        artist_name: "artist 1",
        favourited: false
    },
    {
        key: 6,
        cover: artist3,
        artist_name: "artist 1",
        favourited: true
    },
    {
        key: 7,
        cover: artist2,
        artist_name: "artist 1",
        favourited: true
    },
    {
        key: 8,
        cover: artist1,
        artist_name: "artist 1",
        favourited: false
    },
    {
        key: 9,
        cover: artist1,
        artist_name: "artist 1",
        favourited: true
    },
    {
        key: 10,
        cover: artist1,
        artist_name: "artist 1",
        favourited: false
    },
    {
        key: 11,
        cover: artist1,
        artist_name: "artist 1",
        favourited: false
    },
    {
        key: 12,
        cover: artist1,
        artist_name: "artist 1",
        favourited: false
    },
    {
        key: 13,
        cover: artist1,
        artist_name: "artist 1",
        favourited: false
    },
    {
        key: 14,
        cover: artist1,
        artist_name: "artist 1",
        favourited: false
    },
    {
        key: 15,
        cover: artist1,
        artist_name: "artist 1",
        favourited: false
    },
    {
        key: 16,
        cover: artist1,
        artist_name: "artist 1",
        favourited: false
    },
    {
        key: 17,
        cover: artist1,
        artist_name: "artist 1",
        favourited: false
    },
]

const UserFavorites = () => {
    const [co_ords, setCoords] = useState<mouse_coOrds>({xPos: 0, yPos: 0});
    const [artistMenuToOpen, setArtistMenuToOpen] = useState<artistDetails | null>(null);

    function setMenuOpenData(key: number, n_co_ords: {xPos: number; yPos: number;}){
        setCoords(n_co_ords);
        const matching_artist = artists.find(artist => { return artist.key === key; })
        setArtistMenuToOpen(matching_artist ? matching_artist : null);
    }

    return (
        <div className="UserFavorites">
            <div className="UserFavorites-container">
                {artists.map((artist) => 
                        <SquareTitleBox 
                            key={artist.key}
                            cover={artist.cover} 
                            title={artist.artist_name}
                            keyV={artist.key}
                            setMenuOpenData={setMenuOpenData}
                        />)}
            </div>
            {
                artistMenuToOpen && (
                    <div className="UserFavorites-ContextMenu-container" 
                    onClick={() => {
                        setArtistMenuToOpen(null);
                        setCoords({xPos: 0, yPos: 0});
                    }} 
                    onContextMenu={(e) => {
                        e.preventDefault();
                        setArtistMenuToOpen(null);
                        setCoords({xPos: 0, yPos: 0});
                    }}
                    >
                        <GeneralContextMenu 
                            xPos={co_ords.xPos} 
                            yPos={co_ords.yPos} 
                            title={artistMenuToOpen.artist_name} 
                            favourited={artistMenuToOpen.favourited}
                            CMtype={contextMenuEnum.ArtistCM}/>
                    </div>
                )
            }
        </div>
    )
}

export default UserFavorites