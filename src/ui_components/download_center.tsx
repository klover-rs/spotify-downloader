import { useEffect, useState } from 'react';
import './download_center.css';
import { listen } from '@tauri-apps/api/event';

interface DownloadState {
    status: 'Downloading' | 'Encoding' | 'Downloaded';
    file_name: string;
    downloaded_bytes?: number;
    total_bytes?: number;
}


interface DownloadingStatus {
    Downloading: {
        file_name: string;
        downloaded_bytes: number;
        total_bytes: number;
    }
}

interface EncodingStatus {
    Encoding: {
        file_name: string;
    }
}

interface DownloadedStatus {
    Downloaded: {
        file_name: string;
    }
}

export default function DownloadCenter() {

    const [downloadState, setDownloadState] = useState<DownloadState[]>([]);

    useEffect(() => {

        const getDownloadingSongs = async () => {
            try {
                await listen('download_state', (event) => {
                    const data = JSON.parse(String(event.payload));

                    let newStatus: DownloadState | null = null;

                    if (data.Downloading) {
                        newStatus = {
                            status: 'Downloading',
                            file_name: data.Downloading.file_name,
                            downloaded_bytes: data.Downloading.downloaded_bytes,
                            total_bytes: data.Downloading.total_bytes,
                        };
                    } else if (data.Encoding) {
                        newStatus = {
                            status: 'Encoding',
                            file_name: data.Encoding.file_name,
                        };
                    } else if (data.Downloaded) {
                        newStatus = {
                            status: 'Downloaded',
                            file_name: data.Downloaded.file_name,
                        };
                    }

                    if (newStatus) {
                        setDownloadState((prev) => {
                            const existingIndex =prev.findIndex(item => item.file_name === newStatus.file_name);
                            if (existingIndex !== -1) {
                                const updatedState = [...prev];
                                updatedState[existingIndex] = newStatus;
                                return updatedState;
                            } else {
                                return [...prev, newStatus];
                            }
                        })
                    }
                    
                });
            } catch (e) {
                console.error(e);
            }
        }

        getDownloadingSongs();

    }, []);

    return (
        <div className="download-center">
            {downloadState.map((state, index) => (
                <div key={index} className='download-item'>
                    <h3>{state.file_name}</h3>
                    {state.status === 'Downloading' && (
                        <>
                            <p>
                                Downloading: {Math.round((Number(state.downloaded_bytes) / 1000) / 1000)} / {Math.round((Number(state.total_bytes) / 1000) / 1000)} Mb
                            </p>
                            <div className='progress-bar'>
                                <div 
                                    className='progress'
                                    style={{ width: `${(state.downloaded_bytes! / state.total_bytes!) * 100}%`}}
                                />
                            </div>
                        </>
                  
                    )}
                    {state.status === 'Encoding' && (
                        <p>Encoding...</p>
                    )}
                    {state.status === 'Downloaded' && (
                        <p>Downloaded!</p>
                    )}
                </div>
            ))}
        </div>
    );
}