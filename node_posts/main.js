const Hapi=require('@hapi/hapi')
require('dotenv').config()

const server = Hapi.Server({
    port: 3001
})

server.route({
    path: '/posts',
    method: 'GET',
    async handler(request, h) {
        const client = request.pg.client
        const select = "select * from posts where published='t'"
        let result
        try {
        result = await client.query(select)
        } catch(err) {
            console.error(err)
            return err
        }
        // console.log(result.rows)
        // return `got ${result.rows.length} posts`
        return result.rows
    }
})

const init = async() => {
    await server.register({
        plugin: require('hapi-postgres-connection')
    })
    await server.start()
    console.log(`server started at ${server.info.uri}`)
}

init()