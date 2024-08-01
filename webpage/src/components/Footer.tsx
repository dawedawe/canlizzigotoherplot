import { FunctionComponent, ReactElement } from 'react';
import React = require('react');

const Footer: FunctionComponent = (): ReactElement => {

    let component = () => {
        console.log('I am a footer');
        return (
            <footer>
                <h4>Made with <span title='Love'>❤️</span>
                    <img src={require('../rust.png')} className='footer-image' alt="Rust logo" title='Rust' />
                    <img src={require('../react.svg')} className='footer-image' alt="React logo" title='React' />
                </h4>
            </footer>
        );
    }
    return component();
};

export { Footer };
