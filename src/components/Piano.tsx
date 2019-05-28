import * as React from 'react'
import 'react-piano/dist/styles.css'
import './piano.css'
// @ts-ignore
import { Piano as ReactPiano } from 'react-piano'
// @ts-ignore
import { Player } from '../native/src/lib.rs'

const noteRange = {
  first: 48,
  last: 77
}

const Piano: React.FunctionComponent = () => {
  let player: Player | null

  const playNote = (note: number) => {
    if (!player) player = new Player()
    player.setGain(0.5)
    player.setNote(note)
  }

  const stopNote = () => {
    player.stop()
  }

  const [width, setWidth] = React.useState(0)

  const updateWidth = () => {
    setWidth(window.innerWidth - 40)
  }

  React.useEffect(() => {
    updateWidth()

    window.addEventListener('resize', updateWidth)

    return () => {
      window.removeEventListener('resize', updateWidth)
    }
  })

  return (
    <ReactPiano
      width={width}
      noteRange={noteRange}
      playNote={playNote}
      stopNote={stopNote}
    />
  )
}

export default Piano
