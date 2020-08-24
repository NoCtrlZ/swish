const Client = require('hear_me_roar').default
const client = new Client('localhost', 3000)

describe("Swish Swish Integration Test", () => {
    it("Normal Get Request", () => {
        client.get('/path').then(res => {
            const { status, data } = res
            expect(status).toStrictEqual(true)
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
        });
    })
})
