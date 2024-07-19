import { FunctionComponent, ReactElement, useEffect, useState } from 'react';
import React = require('react');
import { TodayCheck } from '../components/TodayCheck';

const App: FunctionComponent = (): ReactElement => {

    return (
        <main>
            <h1>My React App!</h1>
            <p>This is a simple React app.</p>
            <TodayCheck />
        </main>
    );
};

export { App };
