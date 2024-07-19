import { FunctionComponent, ReactElement } from 'react';
import React = require('react');

const check = () => {
    // a function that fetches a website from a url
    let url = 'https://www.function-type.de';
    fetch(url).then(res => {
        if (res.ok) {
            console.log(res.body);
        } else {
            console.log('Error');
        }
    });
}


const TodayCheck: FunctionComponent = (): ReactElement => {
    // check();
    let today = new Date();
    let component = () => {
        if (today.getSeconds() % 2 === 0) {
            return (
                <p>She can because even</p>
            );
        } else {
            return (
                <p>She can't because odd</p>
            );
        }
    };

    return (
        component()
    );
};

export { TodayCheck };
