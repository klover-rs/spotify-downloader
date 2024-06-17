import { dialog, invoke } from '@tauri-apps/api';
import React from 'react';
import './FilePicker.css';

const FilePickerComponent: React.FC = () => {
    
    const handleFileSelection = async () => {
        const result = await dialog.open({
            directory: true,
            multiple: false
        }).catch(console.error);
        await invoke('set_directory', {
            filePath: result
        })
    }

    return (
        <div>
            <button className='file-picker-btn' onClick={handleFileSelection}>Choose Directory</button>
        </div>
    )
}

export default FilePickerComponent;