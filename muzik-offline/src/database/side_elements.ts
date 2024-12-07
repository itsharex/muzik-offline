export type viewableSideEl = {
    All_tracks: boolean,
    All_artists: boolean,
    All_albums: boolean,
    All_genres: boolean,
    All_playlists: boolean,
};

export const viewableSideElements: viewableSideEl = {
    All_tracks: true,
    All_artists: true,
    All_albums: true,
    All_genres: true,
    All_playlists: true,
}

export function resetViewableElements(obj: viewableSideEl){
    obj.All_tracks = true;
    obj.All_artists = true;
    obj.All_albums = true;
    obj.All_genres = true;
    obj.All_playlists = true;

    return obj;
}