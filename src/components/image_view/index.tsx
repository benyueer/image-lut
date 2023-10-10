import React, { Ref, RefObject, WheelEvent, MouseEvent, useEffect, useRef, useState } from 'react'
import styles from './styles.module.less'

interface Props {
  src: string
}

interface Pos {
  x: number,
  y: number
}

let WIDTH = 500
let HEIGHT = 300


export default function ImageView(props: Props) {
  // const [src, setSrc] = useState('src/assets/input.JPG')
  const [scale, setScale] = useState(1)
  const [offsetDis, setOffsetDis] = useState({ x: 0, y: 0 })
  const [canMove, traggleMove] = useState(false)
  const moveStatePos = useRef<Pos | null>(null)
  const moveStateOff = useRef<Pos | null>(null)

  const [imageDom, setImageDom] = useState(new Image)

  const canvasRef = useRef<HTMLCanvasElement>(null)

  const scrollHandler = (event: WheelEvent<HTMLCanvasElement>) => {
    event.stopPropagation()

    let oldScale = scale
    let newScale = 1
    const delta = event.deltaY
    if (delta > 0 && scale < 10) {
      newScale = scale * 1.1
    } else if (delta < 0 && scale > 0.1) {
      newScale = scale / 1.1
    } else {
      return
    }
    setScale(newScale)

    const { clientX, clientY } = event

    let mousePointTo = {
      x: (clientX - offsetDis.x) / oldScale,
      y: (clientY - offsetDis.y) / oldScale
    }

    const newPos = {
      x: clientX - (mousePointTo.x * newScale),
      y: clientY - (mousePointTo.y * newScale)
    }

    setOffsetDis({ ...newPos })
  }


  function initImage(src: string) {
    imageDom.onload = () => {
      const { width, height, naturalHeight, naturalWidth } = imageDom
      const imageScale = width / naturalWidth
      imageDom.height = naturalHeight * imageScale

      const left = WIDTH / 2 - width / 2
      const top = HEIGHT / 2 - imageDom.height / 2

      setScale(1)
      setOffsetDis({
        x: left,
        y: top
      })
    }
    imageDom.src = src
    imageDom.width = WIDTH
  }

  function mousedownHandler(e: MouseEvent<HTMLCanvasElement>) {
    moveStatePos.current = {
      x: e.clientX,
      y: e.clientY
    }

    moveStateOff.current = offsetDis

    traggleMove(true)

  }

  function mousemoveHandler(e: MouseEvent<HTMLCanvasElement>) {
    // console.log(e)
    if (canMove) {
      const { clientX, clientY } = e
      const offset = {
        x: moveStateOff.current!.x + clientX - moveStatePos.current!.x,
        y: moveStateOff.current!.y + clientY - moveStatePos.current!.y,
      }

      setOffsetDis(offset)
    }
  }

  function mouseupHandler() {
    traggleMove(false)
  }

  function getCanvasSize() {
    const { width, height } = canvasRef.current!.getBoundingClientRect()

    WIDTH = width
    HEIGHT = height
  }

  useEffect(() => {
    getCanvasSize()
  }, [])

  useEffect(() => {
    initImage(props.src)

    const resizeHandler = () => {
      getCanvasSize()
      setScale(1)
    }

    window.addEventListener('resize', resizeHandler)

    return () => {
      window.removeEventListener('resize', resizeHandler)
    }
  }, [imageDom, props])

  useEffect(() => {
    drawImage()
  }, [scale, offsetDis])

  function drawImage() {
    const canvas = canvasRef.current!
    const ctx = canvas.getContext('2d')!
    const { width, height, naturalHeight, naturalWidth } = imageDom

    ctx?.clearRect(0, 0, WIDTH, HEIGHT)
    ctx?.drawImage(imageDom, offsetDis.x, offsetDis.y, width * scale, height * scale)
  }


  return (
    <div className={styles.main}>
      <canvas
        width={WIDTH}
        height={HEIGHT}
        ref={canvasRef}
        onWheel={scrollHandler}
        onMouseDown={mousedownHandler}
        onMouseMove={mousemoveHandler}
        onMouseUp={mouseupHandler}
      ></canvas>
    </div>
  )
}
