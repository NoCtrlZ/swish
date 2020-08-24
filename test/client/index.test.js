const Client = require('hear_me_roar').default;
const client = new Client('server', 3000);

describe('Swish Swish Integration Test', () => {
    it('Static Get Request', async () => {
        const res = await client.get('/path').catch(e => console.log(e))
        const { status, data } = res;

        expect(status).toStrictEqual(true);
        expect(data.code).toStrictEqual(200);
        expect(data.data).toStrictEqual('path request');
    });

    it('Dynamic Get Request', async () => {
        const res = await client.get('/user/shinsaku').catch(e => console.log(e))
        const { status, data } = res;

        expect(status).toStrictEqual(true);
        expect(data.code).toStrictEqual(200);
        expect(data.data).toStrictEqual('user id is shinsaku');
    });

    it('Normal Post Request', async () => {
        const sample = {
            code: 200,
            data: 'hello'
        };

        const res = await client.post('/user/register', sample)
        const { status, data } = res;

        expect(status).toStrictEqual(true);
        expect(data.code).toStrictEqual(200);
        expect(data.data).toStrictEqual(
            `success register id: ${sample.code} msg: ${sample.data}`
        );
    });
});
