import * as React from 'react';

export interface ErrorState {
    errorOccurred: boolean,
    errorInfo: object,
    error: object,
}

export class ErrorWrapper extends React.Component<{}, ErrorState> {
    constructor(props) {
        super(props)
        this.state = { errorOccurred: false, errorInfo: null, error: null }
    }

    componentDidCatch(error, info) {
        this.setState({ errorOccurred: true, errorInfo: info, error: error })
    }

    render() {
        if (this.state.errorOccurred) {
            console.log(this.state.error);
            console.log(this.state.errorInfo);
            return <>
                <div className="text-center">
                    <p>An error occured! code isn't exactly "enterprise" so feel free to tell me on <a className="text-blue-400 hover:text-blue-600" href="https://github.com/camas/reddit-search/issues/new">Github</a> or use <a className="text-blue-400 hover:text-blue-600" href="https://pushshift.io/api-parameters/">pushshift</a> directly</p>
                    <div className="bg-gray-800 overflow-x-auto text-gray-400 mt-8 mx-2">
                        <p>{this.state.error.message}</p>
                        <p className="whitespace-pre-wrap">{this.state.errorInfo.componentStack.trim()}</p>
                    </div>
                </div>
            </>;
        } else {
            return this.props.children;
        }
    }
}
