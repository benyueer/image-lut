import { RouteComponentProps, useNavigate } from '@reach/router'
import React, { useCallback, useMemo, useState } from 'react'
import styles from './styles.module.less'
import { UploadOutlined } from '@ant-design/icons';
import { Button, Upload } from 'antd';
import { open } from '@tauri-apps/api/dialog'
import { invoke } from '@tauri-apps/api/tauri';
// import { setDir } from '@tauri-apps/api/tauri'

interface Props extends RouteComponentProps {

}

export default function DirSelect(props: Props) {
  const [fileList, setFileList] = useState([])
  const navigate = useNavigate();

  const openDirSlector = async () => {
    const files = await open({
      directory: true
    })


    if (!files) return

    await invoke('set_dir', {'newDir': files})
    // const images = await invoke('read_images')
    // console.log(images)

    navigate('lut-view')
  }

  return (
    <div className={styles.main}>
      <Button type="primary" size="large" onClick={openDirSlector}>请选择文件夹</Button>
    </div>
  )
}
