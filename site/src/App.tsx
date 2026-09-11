import Categories from './components/Categories.tsx'
import Entete from './components/Entete.tsx'
import Pied from './components/Pied.tsx'
import Planification from './components/Planification.tsx'
import Principes from './components/Principes.tsx'
import Supports from './components/Supports.tsx'

export default function App() {
  return (
    <>
      <Entete />
      <main>
        <Categories />
        <Planification />
        <Supports />
        <Principes />
      </main>
      <Pied />
    </>
  )
}
