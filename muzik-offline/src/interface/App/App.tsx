import "@styles/App/App.scss";
import { AppMusicPlayer, LeftSidebar, FSMusicPlayer, HeaderLinuxOS, HeaderMacOS, HeaderWindows, NotifyBottomRight } from "@components/index";
import { AllGenres, AllPlaylists, AllTracks, Settings, AlbumDetails, 
  AllAlbums, AllArtists, SearchPage, ArtistCatalogue, GenreView, PlaylistView } from "@pages/index";
import { useEffect, useState } from "react";
import { type } from '@tauri-apps/plugin-os';
import { invoke } from "@tauri-apps/api/core";
import { HashRouter as Router, Routes, Route } from 'react-router-dom';
import { HistoryNextFloating } from "@layouts/index";
import { OSTYPEenum, Payload, toastType } from "@muziktypes/index";
import { AnimatePresence } from "framer-motion";
import { useWallpaperStore, useSavedObjectStore, useIsMaximisedStore, useIsFSStore, usePortStore, useFisrstRunStore, useDirStore, useToastStore } from "@store/index";
import { SavedObject } from "@database/saved_object";
import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/plugin-notification';
import { MiniPlayer } from "@App/index";
import { listen } from "@tauri-apps/api/event";
import { processOSMediaControlsEvent } from "@utils/OSeventControl";
import { fetch_library, getWallpaperURL } from "@utils/index";
import { local_songs_db } from "@database/database";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { LogicalSize } from "@tauri-apps/api/dpi";
const appWindow = getCurrentWebviewWindow();

const App = () => {
  const [openMiniPlayer, setOpenMiniPlayer] = useState<boolean>(false);
  const [openSettings, setOpenSettings] = useState<boolean>(false);
  const [FSplayerState, setFSplayerState] = useState<boolean>(false);
  const [FloatingHNState, setFloatingHNState] = useState<boolean>(false);
  const { isMaximised } = useIsMaximisedStore((state) => { return { isMaximised: state.isMaximised}; });
  const {local_store, setStore} = useSavedObjectStore((state) => { return { local_store: state.local_store, setStore: state.setStore}; });
  const { wallpaperUUID } = useWallpaperStore((state) => { return { wallpaperUUID: state.wallpaperUUID,}; });
  const { appFS } = useIsFSStore((state) => { return { appFS: state.isFS}; });
  const { setPort } = usePortStore((state) => { return { port: state.port, setPort: state.setPort}; });
  const { firstRun, setFirstRun } = useFisrstRunStore((state) => { return { firstRun: state.firstRun, setFirstRun: state.setFirstRun}; });
  const { dir, setDir } = useDirStore((state) => { return { dir: state.dir, setDir: state.setDir}; });
  const { setToast } = useToastStore((state) => { return { setToast: state.setToast }; });

  function closeSetting(){if(openSettings === true)setOpenSettings(false);}

  function toggleSettings(){setOpenSettings(!openSettings);}

  function openPlayer(){setFSplayerState(true);}

  function closePlayer(){setFSplayerState(false);}

  function toggleFloatingHNState(){setFloatingHNState(!FloatingHNState);}

  async function checkOSType(){
    const osType = type();
    let temp: SavedObject = local_store;
    temp.OStype = osType.toString();
    if(osType === OSTYPEenum.Linux)temp.AppThemeBlur = false;
    setStore(temp);
  }

  async function checkAndRequestNotificationPermission(){
    let permissionGranted = await isPermissionGranted();
    if(!permissionGranted)await requestPermission();
  }

  function connect_to_discord(){ 
    if(local_store.AppActivityDiscord === "Yes"){
      invoke("allow_connection_and_connect_to_discord_rpc").then().catch(); 
    }
  }

  async function ToggleMiniPlayer(){
    let MPV = openMiniPlayer;
    //if miniplayer is open, set the body and html min height to 376px and min width to 218px
    if(!MPV){
      document.body.style.minHeight = "376px";
      document.body.style.minWidth = "218px";
      document.documentElement.style.minHeight = "376px";
      document.documentElement.style.minWidth = "218px";
    }
    //else set the body and html min height to 623px and min width to 980px
    else{
      document.body.style.minHeight = "623px";
      document.body.style.minWidth = "980px";
      document.documentElement.style.minHeight = "623px";
      document.documentElement.style.minWidth = "980px";
    }
    setOpenMiniPlayer(!openMiniPlayer);
    await invoke("toggle_miniplayer_view", {openMiniPlayer: !MPV});
  }

  async function listenForOSevents(){
    const unlisten = await listen<Payload>('os-media-controls', (event) => processOSMediaControlsEvent(event.payload))
    // later, when you want to stop listening
    return unlisten
  }

  async function get_server_port(){
    const port: any = await invoke("get_server_port");
    setPort(port);
  }

  async function check_paths_for_new_music(){
    let paths = dir.Dir;
    if(firstRun){
      const audio_dir: any = await invoke("get_audio_dir");
      // append the audio directory to the directories array
      if(paths.includes(audio_dir) === false){
        setDir({Dir: [...dir.Dir, audio_dir]});
        paths.push(audio_dir);
      }
      setFirstRun(false);
    }
    invoke("refresh_paths", { pathsAsJsonArray: JSON.stringify(paths), compressImageOption: local_store.CompressImage === "Yes" ? true : false })
    .then(async(response: any) => {
      if(response === "No new songs detected")return;
      const res = await fetch_library(false);
      let message = "";

      if(res.status === "error")message = res.message;
      else message = "Successfully loaded all the songs in the paths specified. You may need to reload the page you are on to see your new songs";

      setToast({title: "Loading songs...", message: message, type: toastType.success, timeout: 5000});

      const permissionGranted = await isPermissionGranted();
      if(permissionGranted)sendNotification({ title: 'Loading songs...', body: message });
    });
  }

  async function check_if_paths_are_still_valid(){
    invoke("detect_deleted_songs")
    .then(async(response: any) => {// response is a json array of the uuids of the songs that were deleted
      local_songs_db.songs.bulkDelete(response)
    });
  }

  async function changeMouseToResizeCursor(e: MouseEvent){
    // if user goes within 2px of edge of window, the cursor will change to a resize cursor
    const isFS = await appWindow.isFullscreen();
    const isMaximised = await appWindow.isMaximized();
    if(!openMiniPlayer && !isFS && !isMaximised){
      if(e.clientX <= 1 || e.clientX >= window.innerWidth - 1 || e.clientY <= 1 || e.clientY >= window.innerHeight - 1){
        if(e.clientX <= 1 || e.clientX >= window.innerWidth - 1){//if dragging from the left or right edge of the window
          document.body.style.cursor = "ew-resize";
        } else if(e.clientY <= 1 || e.clientY >= window.innerHeight - 1){//if dragging from the top or bottom edge of the window
          document.body.style.cursor = "ns-resize";
        } else if(e.clientX <= 1 && e.clientY <= 1){//if dragging from the top left corner of the window
          document.body.style.cursor = "nw-resize";
        } else if(e.clientX >= window.innerWidth - 1 && e.clientY <= 1){//if dragging from the top right corner of the window
          document.body.style.cursor = "ne-resize";
        } else if(e.clientX <= 1 && e.clientY >= window.innerHeight - 1){//if dragging from the bottom left corner of the window
          document.body.style.cursor = "sw-resize";
        } else if(e.clientX >= window.innerWidth - 1 && e.clientY >= window.innerHeight - 1){//if dragging from the bottom right corner of the window
          document.body.style.cursor = "se-resize";
        }
      }
      else{
        document.body.style.cursor = "auto";
      }
    }
  }

  async function windowResizeHandler(e: MouseEvent){
    if(!openMiniPlayer){
      // if user goes within 1px of edge of window and clicks and drags, the window will resize
      if(e.clientX <= 1 || e.clientX >= window.innerWidth - 1 || e.clientY <= 1 || e.clientY >= window.innerHeight - 1){
        const physicalSize = await appWindow.innerSize();
        if(e.clientY <= 1){//if dragging from the top edge of the window
          appWindow.setSize(new LogicalSize(physicalSize.width, physicalSize.height - e.movementY));
        } else if(e.clientY >= window.innerHeight - 1){//if dragging from the bottom edge of the window
          appWindow.setSize(new LogicalSize(physicalSize.width, physicalSize.height + e.movementY));
        } else if(e.clientX <= 1){//if dragging from the left edge of the window
          appWindow.setSize(new LogicalSize(physicalSize.width - e.movementX, physicalSize.height));
        } else if(e.clientX >= window.innerWidth - 1){//if dragging from the right edge of the window
          appWindow.setSize(new LogicalSize(physicalSize.width + e.movementX, physicalSize.height));
        } else if(e.clientX <= 1 && e.clientY <= 1){//if dragging from the top left corner of the window
          appWindow.setSize(new LogicalSize(physicalSize.width - e.movementX, physicalSize.height - e.movementY));
        } else if(e.clientX >= window.innerWidth - 1 && e.clientY <= 1){//if dragging from the top right corner of the window
          appWindow.setSize(new LogicalSize(physicalSize.width + e.movementX, physicalSize.height - e.movementY));
        } else if(e.clientX <= 1 && e.clientY >= window.innerHeight - 1){//if dragging from the bottom left corner of the window
          appWindow.setSize(new LogicalSize(physicalSize.width - e.movementX, physicalSize.height + e.movementY));
        } else if(e.clientX >= window.innerWidth - 1 && e.clientY >= window.innerHeight - 1){//if dragging from the bottom right corner of the window
          appWindow.setSize(new LogicalSize(physicalSize.width + e.movementX, physicalSize.height + e.movementY));
        }
      }
    }
  }

  useEffect(() => {
    checkOSType();
    checkAndRequestNotificationPermission();
    connect_to_discord();
    get_server_port();
    check_paths_for_new_music();
    check_if_paths_are_still_valid();
    window.addEventListener("mousemove", changeMouseToResizeCursor);
    window.addEventListener("mousedown", windowResizeHandler);
    const listenForOSeventsfunc = listenForOSevents();

    return () => {
      window.removeEventListener("mousemove", changeMouseToResizeCursor);
      window.removeEventListener("mousedown", windowResizeHandler);
      listenForOSeventsfunc.then((unlisten) => unlisten());
    }
  }, [])

  return (
    <>
    { !openMiniPlayer ?
      <Router>
        <div 
          className={
            "app_container " + 
            (local_store.OStype === OSTYPEenum.Windows && ((!appFS && !isMaximised) || local_store.AlwaysRoundedCornersWindows === "Yes") ? " windows-app-config " : "") +
            (local_store.OStype === OSTYPEenum.Linux || !local_store.AppThemeBlur ? " linux-config " : "")
            
          } 
          data-theme={local_store.ThemeColour} 
          wallpaper-opacity={local_store.WallpaperOpacityAmount}
          onContextMenu={(e) => e.preventDefault()}>
            <div className={"background_img " + (wallpaperUUID ? "" : local_store.BGColour)}>
              {wallpaperUUID && (<img src={getWallpaperURL(wallpaperUUID)} alt="wallpaper"/>)}
            </div>
            <div className={"app_darkness_layer " + (wallpaperUUID ? "image_layer" : "color_layer")}>
              {
                local_store.OStype ===  OSTYPEenum.Windows ? 
                  <HeaderWindows toggleSettings={toggleSettings}/>
                  :
                  local_store.OStype ===  OSTYPEenum.macOS ? 
                  <HeaderMacOS toggleSettings={toggleSettings}/> 
                  :
                  <HeaderLinuxOS toggleSettings={toggleSettings}/>
              }
              <div className="main_app_container">
                <div className="left_sidebar">
                  <LeftSidebar />
                </div>
                <div className="center_activity">
                      <AnimatePresence>
                        <Routes>
                              <Route path="/" element={<AllTracks/>}/>
                              <Route path="/AllArtists" element={<AllArtists/>}/>
                              <Route path="/AllAlbums" element={<AllAlbums/>}/>
                              <Route path="/AllGenres" element={<AllGenres/>}/>
                              <Route path="/AllPlaylists" element={<AllPlaylists/>}/>
                              <Route path="/SearchPage/*" element={<SearchPage/>}/>
                              <Route path="/AlbumDetails/:album_key/:artist_name" element={<AlbumDetails/>}/>
                              <Route path="/ArtistCatalogue/:artist_name" element={<ArtistCatalogue/>}/>
                              <Route path="/GenreView/:genre_key" element={<GenreView/>}/>
                              <Route path="/PlaylistView/:playlist_key" element={<PlaylistView/>}/>
                        </Routes>
                      </AnimatePresence>
                </div>
              </div>
                <AppMusicPlayer openPlayer={openPlayer} toggleFloatingHNState={toggleFloatingHNState} openMiniPlayer={ToggleMiniPlayer}/>
                <Settings openSettings={openSettings} closeSettings={closeSetting}/>
                <FSMusicPlayer openPlayer={FSplayerState} closePlayer={closePlayer}/>
                <HistoryNextFloating FloatingHNState={FloatingHNState} toggleFloatingHNState={toggleFloatingHNState}/>
                <NotifyBottomRight/>
            </div>
        </div>
      </Router>
      :
      <MiniPlayer isOpen={openMiniPlayer} closeMiniPlayer={ToggleMiniPlayer}/>
    }
    </>
  );
}

export default App
