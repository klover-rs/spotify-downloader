import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import { useNavigate } from "react-router-dom";
import DownloadCenter from "./ui_components/download_center";
import { listen } from "@tauri-apps/api/event";
import FilePickerComponent from "./ui_components/FilePicker";

function App() {

  const nav = useNavigate();
  const [error, setError] = useState("");

  useEffect(() => {

    const checkCreds = async () => {
      try {
        await invoke("is_logged_in");

      } catch (e) {
        console.error(e);
        nav("/login", { replace: true });
      }
    }

    checkCreds();


  }, [])

  const [spotifyUrl, setSpotifyUrl] = useState("");
  const [disableBtn, setDisableBtn] = useState(false);

  const downloadTrack = async () => {
    try {
      let result = await invoke("download_tracks", {
        url: spotifyUrl
      });
      console.log(result);
      console.log("finished download!");
      setDisableBtn(false);
    } catch (e) {
      console.error(e);
      setError(String(e));
    }
  }

  return (
    <div className="container">
      <h1 className="welcome-header">Welcome to spotify downloader!</h1>
      <div className="dl-tracks-container">
        <div className="dl-tracks-container-inner">
          <FilePickerComponent />
          <br/>
          <label>Supports: Tracks, Playlists, Albums</label>
          <br/>
          <input
            className="spotify-url-input"
            type='text'
            placeholder="spotify share url here" 
            value={spotifyUrl}
            onChange={(e) => setSpotifyUrl(e.target.value)}
          />
          <br/>
          <button className="dl-btn" onClick={() => {
          downloadTrack();
          setDisableBtn(prevValue => !prevValue);
          }} disabled={disableBtn}>download track(s)</button>
          <h4>{error}</h4>
        </div>

      </div>
      <div>
        <DownloadCenter />
      </div>
    </div>
  );
}

export default App;
