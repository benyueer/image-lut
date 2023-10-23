import { RouteComponentProps } from '@reach/router'
import { invoke } from '@tauri-apps/api/tauri';
import styles from './styles.module.less'
import React, { useEffect, useState } from 'react'
import ImageView from '../../components/image_view';
import { Radio, Spin } from 'antd'

interface Props extends RouteComponentProps { }


export default function MainView(props: Props) {
  const [images, setImages] = useState([])
  const [loading, setLoading] = useState(true)
  const [curLut, setLut] = useState('Default')
  const [state, setState] = useState({
    curIndex: -1,
    // srcs: []
  })
  console.log(1111, images, state, curLut, loading)

  useEffect(() => {
    const getImages = async () => {
      const i = await invoke('read_images')
      const imagesMap = i.map(({ name, data }) => ({
        name,
        srcs: {
          Default: `data:image/jpg;base64,${data}`
        }
      }))
      setImages(imagesMap)
      setState({
        curIndex: 0
      })
      setLoading(false)
    }

    getImages()
  }, [])

  const handler = (e) => {
    switch (e.key) {
      case 'ArrowLeft':
        changeIndex(-1)
        break;
      case 'ArrowRight':
        changeIndex(1)
        break;
    }
  }

  useEffect(() => {
    document.addEventListener('keydown', handler)

    return () => {
      document.removeEventListener('keydown', handler)
    }
    // ⚠️ 为什么要加这个依赖？ useEffect 在依赖不变时不会重复执行，那么当我们设置事件回调等类似函数时，函数使用了外部state，就会是旧的闭包，获取不到新值
    // 因此要记住一个原则，最好将 useEffect 使用到的外部依赖都加到依赖数组里，避免奇怪的BUG
  }, [state.curIndex])


  const changeIndex = (to: number) => {
    const curIndex = (state.curIndex + to + images.length) % images.length
    setState({
      // ...state,
      curIndex
    })

    const el = document.querySelector(`#btm-img-${curIndex}`)
    el.scrollIntoView()
  }

  const btmImgClickHandler = (index: number) => {
    setState({
      // ...state,
      curIndex: index
    })
  }

  useEffect(() => {
    const fun = async () => {
      if (state.curIndex < 0) return
      setLoading(true)
      const imgs = [...images]
      const curImage = imgs[state.curIndex]
      console.log(1)
      if (!curImage?.srcs[curLut]) {
        console.log(2)
        const src = await invoke('get_lut_image', { imagePath: curImage.name, lutName: curLut })
        console.log(3)
        curImage.srcs[curLut] = `data:image/jpg;base64,${src}`

        setImages(imgs)
        console.log(4, imgs)
      }
      setLoading(false)
    }

    fun()
  }, [curLut, state.curIndex])

  return (
    <div className={styles.main}>
      {
        !loading ?
          <ImageView src={images[state.curIndex].srcs[curLut]}></ImageView>
          : <div className={styles.loadingWrap}>
            <Spin size="large" />
          </div>
      }

      <div className={styles.bottomWrap}>
        {
          images.map((img, index) => <img
            key={img.name}
            id={`btm-img-${index}`}
            src={img.srcs.Default}
            className={index === state.curIndex ? styles.curImage : ''}
            onClick={() => btmImgClickHandler(index)}
          ></img>)
        }
      </div>
      <div className={styles.topWrap}>
        <Radio.Group value={curLut} onChange={({ target: { value } }) => setLut(value)} buttonStyle="solid">
          <Radio.Button value="Default">默认</Radio.Button>
          <Radio.Button value="Fashion">fashion</Radio.Button>
          <Radio.Button value="HiCon">HiCon</Radio.Button>
          {/* <Radio.Button value="c">Beijing</Radio.Button>
          <Radio.Button value="d">Chengdu</Radio.Button> */}
        </Radio.Group>
      </div>
    </div>
  )
}


function u8ToUrl(imageData) {
  console.log(imageData.slice(0, 10))
  const blob = new Blob([Uint8Array.from(imageData)], { type: 'image/png' })
  const imageUrl = URL.createObjectURL(blob)
  return imageUrl
}