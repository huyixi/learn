import { CheerioCrawler } from 'crawlee';

const crawler = new CheerioCrawler({
    async requestHandler({ $, request,enqueueLinks }) {
        const title = $('li').text();
        console.log(`The title of "${request.url}" is: ${title}.`);
        await enqueueLinks('a');
    }
})

await crawler.run(["https://huyixi.org/journal/"]);
