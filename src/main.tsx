import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App.tsx'
import './styles.css'

const racine = document.getElementById('root')
if (!racine) throw new Error('#root introuvable')

ReactDOM.createRoot(racine).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
)
