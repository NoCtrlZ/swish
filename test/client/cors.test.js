const Client = require('hear_me_roar').default;
const client = new Client('localhost', 3000);

describe('Swish Swish Integration Test', () => {
    it('Static Get Request', async () => {
        const res = await client.get('/path').catch(e => console.log(e))
        const { status, data } = res;
        console.log(res)

        expect(status).toStrictEqual(false);
        expect(data.code).toStrictEqual(401);
        expect(data.message).toStrictEqual('header is not accepted');
    });

    it('Dynamic Get Request', async () => {
        const res = await client.get('/user/shinsaku').catch(e => console.log(e))
        const { status, data } = res;

        expect(status).toStrictEqual(false);
        expect(data.code).toStrictEqual(401);
        expect(data.message).toStrictEqual('header is not accepted');
    });

    it('Normal Post Request', async () => {
        const sample = {
            code: 401,
            data: 'hello'
        };

        const res = await client.post('/user/register', sample)
        const { status, data } = res;

        expect(status).toStrictEqual(false);
        expect(data.code).toStrictEqual(401);
        expect(data.message).toStrictEqual('header is not accepted');
    });
});
