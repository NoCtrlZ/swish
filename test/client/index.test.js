const Client = require('hear_me_roar').default
const client = new Client('localhost', 3000)

describe("Swish Swish Integration Test", () => {
    it("Static Get Request", () => {
        client.get('/path').then(res => {
            const { status, data } = res

            expect(status).toStrictEqual(true)
            expect(data.code).toStrictEqual(200)
            expect(data.data).toStrictEqual("path request")
        })
    })

    it("Dynamic Get Request", () => {
        client.get('/user/shinsaku').then(res => {
            const { status, data } = res

            expect(status).toStrictEqual(true)
            expect(data.code).toStrictEqual(200)
            expect(data.data).toStrictEqual("user id is shinsaku")
        })
    })

    it("Normal Post Request", () => {
        const sample = {
            code: 200,
            data: 'hello'
        };

        client.post('/user/register', sample).then((res) => {
            const { status, data } = res

            expect(status).toStrictEqual(true)
            expect(data.code).toStrictEqual(200)
            expect(data.data).toStrictEqual(`success register id: ${sample.code} msg: ${sample.data}`)
        });
    })
})
