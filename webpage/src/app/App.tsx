import { FunctionComponent, ReactElement, useEffect, useState } from 'react';
import React = require('react');
import { TodayCheck } from '../components/TodayCheck';
import { Upcoming } from '../components/Upcoming';
import { Footer } from '../components/Footer';

const App: FunctionComponent = (): ReactElement => {

    return (
        <main>
            <h1>🌻 Can Lizzi go to her plot today? 🌾</h1>
            <TodayCheck />
            <Upcoming />
            <Footer />
        </main>
    );
};

export { App };
