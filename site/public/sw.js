// Service worker d'extinction.
//
// Jusqu'ici l'application vivait à la racine, et son service worker — enregistré
// sous cette même adresse, /sw.js — contrôle « / » chez toutes les personnes qui
// l'ont déjà ouverte. Il répondrait l'application pour cette URL, quoi que le
// serveur y place désormais : le site de présentation ne s'afficherait jamais.
//
// Les navigateurs revérifient le script à son adresse d'origine. Ce fichier
// prend donc la place de l'ancien, vide les caches, se désinscrit, puis
// recharge les onglets ouverts — qui reçoivent alors le site.
//
// Aucune page ne l'enregistre : seuls les navigateurs ayant déjà l'ancien
// service worker le reçoivent. Il est à conserver tant que des installations
// d'avant la migration peuvent subsister.

self.addEventListener('install', () => {
  self.skipWaiting()
})

self.addEventListener('activate', event => {
  event.waitUntil((async () => {
    for (const nom of await caches.keys()) await caches.delete(nom)
    await self.registration.unregister()
    for (const client of await self.clients.matchAll({ type: 'window' })) {
      client.navigate(client.url)
    }
  })())
})
