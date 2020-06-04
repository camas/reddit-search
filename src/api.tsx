export enum SearchType {
  Comments,
  Posts,
}

export interface SearchSettings {
  author: string,
  subreddit: string,
  searchFor: SearchType,
  resultSize: number,
  filter: string,
  after: Date,
  before: Date,
  query: string,
}

export class PushshiftAPI {
  get_url(settings: SearchSettings): string {
    let args = {
      html_decode: "true",
    };
    if (settings.after) {
      args["after"] = (settings.after.valueOf() / 1000).toString();
    }
    if (settings.before) {
      args["before"] = (settings.before.valueOf() / 1000).toString();
    }
    if (settings.author) {
      args["author"] = settings.author;
    }
    if (settings.subreddit) {
      args["subreddit"] = settings.subreddit;
    }
    if (settings.query) {
      args["q"] = settings.query;
    }
    if (settings.resultSize) {
      args["size"] = settings.resultSize.toString();
    }
    if (settings.filter) {
      args["score"] = settings.filter;
    }
    let joinedArgs = Object.entries(args).map(([k, v]) => `${k}=${v}`).join('&');
    let endpoint;
    if (settings.searchFor == SearchType.Comments) {
      endpoint = "comment";
    }
    else if (settings.searchFor == SearchType.Posts) {
      endpoint = "submission";
    }
    return `https://api.pushshift.io/reddit/${endpoint}/search?${joinedArgs}`
  }

  async query(url: string): Promise<any> {

    console.log(`Pushshift request ${url}`);
    let resp = await fetch(url, {
      referrerPolicy: "no-referrer"
    });
    let data = await resp.json();
    return data;
  }
}
