import * as React from 'react';
import * as ReactDOM from 'react-dom';
import { App } from './app';
import { ErrorWrapper } from './error';

ReactDOM.render(<ErrorWrapper><App /></ErrorWrapper>, document.getElementById('react-root'));
