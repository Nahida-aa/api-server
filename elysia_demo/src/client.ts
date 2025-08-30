import { treaty } from '@elysiajs/eden'
import type { App } from './server'

const app = treaty<App>('localhost:3000')

// const id = await app.id({ id: '12' }).get()




// // Call [GET] at '/'
// const { data } = await app.get()

// Call [PUT] at '/nendoroid/:id'
// const { data: nendoroid, error } = await app.nendoroid({ id: 1895 }).put({
//     name: 'Skadi',
//     from: 'Arknights'
// })