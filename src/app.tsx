import * as React from 'react';
import DatePicker from 'react-datepicker'
import { PushshiftAPI, SearchSettings, SearchType } from './api'
import { GithubLink } from './github-link'

import "react-datepicker/dist/react-datepicker.css";
import { RandomLink } from './random-link';

interface AppState extends SearchSettings {
  error: string,
  errorTime: Date,
  searching: boolean,
  comments: Array<any>,
  posts: Array<any>,
  moreing: boolean,
  lastUrl: string,
}

/** Main class for Reddit Search */
export class App extends React.Component<{}, AppState> {
  lastSearch: SearchSettings;
  api: PushshiftAPI;
  updatedHash: boolean;

  constructor(props) {
    super(props);
    this.state = {
      author: "",
      subreddit: "",
      searchFor: SearchType.Comments,
      resultSize: 100,
      filter: "",
      after: null,
      before: null,
      query: "",
      error: null,
      errorTime: null,
      searching: false,
      comments: null,
      posts: null,
      moreing: false,
      lastUrl: "",
    };
    this.api = new PushshiftAPI();
    this.updatedHash = false;
  }

  loadLocationHash(shouldSearch: boolean = false) {
    let params = hash_accessor.load();
    if (params.after) {
      params.after = new Date(params.after);
    }
    if (params.before) {
      params.before = new Date(params.before);
    }
    if (shouldSearch) {
      this.setState(params, this.doSearch);
    } else {
      this.setState(params);
    }
  }

  componentDidMount() {
    // Add hash change event listener
    window.addEventListener("hashchange", e => {
      if (this.updatedHash) {
        this.updatedHash = false;
        return;
      }
      console.log("location.hash changed. loading new params");
      this.loadLocationHash();
    });

    // Check for location hash. Use it if found
    if (window.location.hash) {
      this.loadLocationHash(true);
      console.log("Loaded params from location.hash");
      return;
    }

    // Load stored form data if exists
    let formDataJson = localStorage.getItem("search-form");
    if (formDataJson !== null) {
      let formData: SearchSettings = JSON.parse(formDataJson);
      // Reconstruct `Date`s
      if (formData.after) {
        formData.after = new Date(formData.after);
      }
      if (formData.before) {
        formData.before = new Date(formData.before);
      }
      this.setState(formData);
      console.log("Loaded params from local storage");
    }
  }

  componentDidUpdate() {
    let toSave: SearchSettings = {
      author: this.state.author,
      subreddit: this.state.subreddit,
      searchFor: this.state.searchFor,
      resultSize: this.state.resultSize,
      filter: this.state.filter,
      after: this.state.after,
      before: this.state.before,
      query: this.state.query,
    };
    localStorage.setItem("search-form", JSON.stringify(toSave));
  }

  setError = (error: string) => {
    this.setState({ error: error, errorTime: new Date() });
  }

  handleAuthorChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    this.setState({ author: e.target.value });
  }

  handleSubredditChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    this.setState({ subreddit: e.target.value });
  }

  handleSearchTypeChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    if (e.target.value === "Comments") {
      this.setState({ searchFor: SearchType.Comments });
    } else if (e.target.value === "Posts") {
      this.setState({ searchFor: SearchType.Posts });
    } else {
      this.setError(e.target.value + " is not a valid search type");
    }
  }

  handleResultSizeChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    let count: number = parseInt(e.target.value);
    if (!count) {
      return;
    }
    this.setState({ resultSize: count });
  }

  handleFilterChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    this.setState({ filter: e.target.value });
  }

  handleAfterDateChange = (date: Date) => {
    this.setState({ after: date });
  }

  handleBeforeDateChange = (date: Date) => {
    this.setState({ before: date });
  }

  handleQueryChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    this.setState({ query: e.target.value });
  }

  doSearch = async () => {
    this.setState({ error: null, comments: null, posts: null, searching: true });
    this.lastSearch = { ...this.state };

    // Update location.hash
    let toSave = {
      author: this.state.author,
      subreddit: this.state.subreddit,
      searchFor: this.state.searchFor,
      resultSize: this.state.resultSize,
      filter: this.state.filter,
      after: this.state.after,
      before: this.state.before,
      query: this.state.query,
    };
    this.updatedHash = true;
    hash_accessor.save(toSave);

    // Search
    let [data, url] = await this.api.query(this.lastSearch);
    this.setState({ lastUrl: url });

    // Update state with results
    if (this.lastSearch.searchFor == SearchType.Comments) {
      this.setState({ comments: data.data, searching: false });
    } else if (this.lastSearch.searchFor == SearchType.Posts) {
      this.setState({ posts: data.data, searching: false });
    }
  }

  /** Handle the main form being submitted */
  searchSubmit = async (e) => {
    // Update state
    e.preventDefault();
    this.doSearch();
  }

  /** Handle the more button being clicked. */
  handleMoreClick = async (e) => {
    this.setState({ error: null, moreing: true });
    if (this.state.comments) {
      this.lastSearch.before = new Date(this.state.comments[this.state.comments.length - 1].created_utc * 1000);
    } else {
      this.lastSearch.before = new Date(this.state.posts[this.state.posts.length - 1].created_utc * 1000);
    }
    let data = await this.api.query(this.lastSearch);
    if (this.lastSearch.searchFor == SearchType.Comments) {
      this.setState({ comments: this.state.comments.concat(data.data), moreing: false });
    } else if (this.lastSearch.searchFor == SearchType.Posts) {
      this.setState({ posts: this.state.posts.concat(data.data), moreing: false });
    }
  }

  /** Render the app
   * @return {React.ReactNode} The react node for the app
   */
  render(): React.ReactNode {
    // Not tidy at all but it's a one page app so WONTFIX
    let moreButton = <button type="button" onClick={this.handleMoreClick} className="bg-red-900 hover:bg-red-800 font-bold py-2 mb-1">{this.state.moreing ? "Moreing..." : "More"}</button>;
    let linkClass = "text-blue-400 hover:text-blue-600";
    let content;
    let resultCount;
    let inner;
    if (this.state.comments) {
      resultCount = this.state.comments.length;
      // Render comments
      inner = this.state.comments.map((comment) => {
        let permalink;
        if (comment.permalink) {
          permalink = comment.permalink;
        } else {
          permalink = `/comments/${comment.link_id.split('_')[1]}/_/${comment.id}`
        }
        return <div className="bg-gray-900 px-1 mb-1" key={comment.id}>
          <div className="flex">
            <a href={`https://reddit.com/r/${comment.subreddit}`}>
              <div className="text-sm text-red-500">/r/{comment.subreddit}</div>
            </a>
            <a href={`https://reddit.com/u/${comment.author}`}>
              <div className="text-sm text-red-500 ml-2">/u/{comment.author}</div>
            </a>
            <div className="text-sm text-red-500 ml-auto">{new Date(comment.created_utc * 1000).toLocaleString()}</div>
          </div>
          <a href={`https://reddit.com${permalink}`}>
            <div className="whitespace-pre-wrap">{comment.body}</div>
          </a>
        </div>
      });
    } else if (this.state.posts) {
      resultCount = this.state.posts.length;
      // Render posts
      inner = this.state.posts.map((post) => {
        let thumbnailUrl;
        if (post.thumbnail.startsWith('http')) {
          thumbnailUrl = post.thumbnail;
        } else if (post.url.split('.').pop() === 'png' || post.url.split('.').pop() === 'jpg') {
          thumbnailUrl = post.url;
        }
        let body;
        if (post.selftext.length !== 0) {
          body = <div className="whitespace-pre-wrap">{post.selftext}</div>
        } else {
          body = <a href={post.url}>
            <div>{post.url}</div>
          </a>
        }
        return <div className="bg-gray-900 px-1 mb-1" key={post.id}>
          <div className="flex">
            <a href={`https://reddit.com/r/${post.subreddit}`}>
              <div className="text-sm text-red-500">/r/{post.subreddit}</div>
            </a>
            <a href={`https://reddit.com/r/${post.author}`}>
              <div className="text-sm text-red-500 ml-2">/u/{post.author}</div>
            </a>
            <div className="text-sm text-red-500 ml-auto">{new Date(post.created_utc * 1000).toLocaleString()}</div>
          </div>
          <div className="flex">
            <div className="mr-1 mb-1 w-32 h-32 overflow-hidden flex-shrink-0">
              <img className="w-full h-full object-cover" src={thumbnailUrl} />
            </div>
            <div>
              <a href={`https://reddit.com/${post.id}`}>
                <div className="font-bold">{post.title}</div>
              </a>
              {body}
            </div>
          </div>
        </div>
      });
    }
    if (this.state.comments || this.state.posts) {
      content = <div className="flex flex-col px-auto max-w-5xl mx-auto">
        <div className="mx-auto mb-1">{resultCount} results - <a className={linkClass} href={this.state.lastUrl}>Generated API URL</a></div>
        {inner}
        {moreButton}
      </div>
    } else {
      content = <div className="text-center px-4 max-w-5xl mx-auto">
        <p>Search reddit using the <a className={linkClass} href="https://pushshift.io/">pushshift.io API</a>. For more advanced searches you can directly query the API <a className={linkClass} href="https://api.pushshift.io/reddit/comment/search?distinguished=admin&q=howdy&subreddit=!ModSupport">fairly easily</a>.</p>
        <p>The 'More' button works by changing the 'before' value to the time of the last post in the results. <em>This means that entries might be missed if they were posted at the same time.</em></p>
      </div >
    }
    // Combine everything and return
    return (
      <>
        <GithubLink />
        <form onSubmit={this.searchSubmit} className="flex flex-col mx-auto max-w-3xl pb-1 mb-3">
          {/* Author and Subreddit */}
          <div className="sm:flex">
            <div className="sm:w-1/2">
              <label className="block text-xs uppercase font-bold">Author</label>
              <input onChange={this.handleAuthorChange} value={this.state.author} className="text-gray-900 bg-gray-300 focus:bg-gray-100 w-full py-2 px-1" />
            </div>
            <div className="sm:w-1/2 sm:ml-1">
              <label className="block text-xs uppercase font-bold">Subreddit</label>
              <input onChange={this.handleSubredditChange} value={this.state.subreddit} className="text-gray-900 bg-gray-300 focus:bg-gray-100 w-full py-2 px-1" />
            </div>
          </div>
          {/* Type, Count and Score Filter */}
          <div className="sm:flex">
            <div className="sm:w-1/3">
              <label className="block text-xs uppercase font-bold">Search for</label>
              <div className="relative">
                <select onChange={this.handleSearchTypeChange} value={this.state.searchFor === SearchType.Comments ? "Comments" : "Posts"} className="text-gray-900 bg-gray-300 focus:bg-gray-100 w-full py-2 px-1 appearance-none">
                  <option>Comments</option>
                  <option>Posts</option>
                </select>
                <div className="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-gray-700">
                  <svg className="fill-current h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20"><path d="M9.293 12.95l.707.707L15.657 8l-1.414-1.414L10 10.828 5.757 6.586 4.343 8z" /></svg>
                </div>
              </div>
            </div>
            <div className="sm:w-1/3 sm:ml-1">
              <label className="block text-xs uppercase font-bold">Num. Returned</label>
              <input onInput={this.handleResultSizeChange}
                className="text-gray-900 bg-gray-300 focus:bg-gray-100 w-full py-2 px-1" type="number" min="25" step="25" value={this.state.resultSize} onChange={e => { }} />
            </div>
            <div className="sm:w-1/3 sm:ml-1">
              <label className="block text-xs uppercase font-bold">Score Filter</label>
              <input onChange={this.handleFilterChange} value={this.state.filter} className="text-gray-900 bg-gray-300 focus:bg-gray-100 w-full py-2 px-1" placeholder="e.g. >10 <100 >100,<900" />
            </div>
          </div>
          {/* Time Range */}
          <div className="sm:flex">
            <div className="sm:w-1/2">
              <label className="block text-xs uppercase font-bold">After</label>
              <DatePicker
                showTimeSelect
                timeFormat="HH:mm"
                timeIntervals={15}
                timeCaption="time"
                dateFormat="MMMM d, yyyy h:mm aa"
                className="text-gray-900 bg-gray-300 focus:bg-gray-100 py-2 px-1"
                onChange={this.handleAfterDateChange}
                selected={this.state.after}
              />
            </div>
            <div className="sm:w-1/2 sm:ml-1">
              <label className="block text-xs uppercase font-bold">Before</label>
              <DatePicker
                showTimeSelect
                timeFormat="HH:mm"
                timeIntervals={15}
                timeCaption="time"
                dateFormat="MMMM d, yyyy h:mm aa"
                className="text-gray-900 bg-gray-300 focus:bg-gray-100 py-2 px-1"
                onChange={this.handleBeforeDateChange}
                selected={this.state.before}
              />
            </div>
          </div>
          {/* Search Term */}
          <div>
            <label className="block text-xs uppercase font-bold">Search Term</label>
            <input onChange={this.handleQueryChange} value={this.state.query} className="text-gray-900 bg-gray-300 focus:bg-gray-100 w-full py-2 px-1" />
          </div>
          {/* Submit Button and Error text */}
          <button type="submit" className="bg-red-900 hover:bg-red-800 font-bold mt-4 py-2">{this.state.searching ? "Searching..." : "Search"}</button>
          {this.state.error &&
            <p className="text-red-200 text-center">{this.state.errorTime.toLocaleTimeString()} Error: {this.state.error}</p>
          }
        </form>
        {content}
        <div className="pb-2 pt-4 text-center"><RandomLink /></div>
      </>
    );
  }
}

// https://gist.github.com/jokester/4a543ea76dbc5ae1bf05
var hash_accessor = (function (window) {
  return {
    load: function () {
      try {
        // strip ^#
        var json_str_escaped = window.location.hash.slice(1);
        // unescape
        var json_str = decodeURIComponent(json_str_escaped);
        return JSON.parse(json_str);
      } catch (e) {
        return {};
      }
    },
    save: function (obj) {
      // use replace so that previous url does not go into history
      window.location.replace('#' + JSON.stringify(obj, (key, value) => { if (value) return value; }));
    }
  };
})(window);
