import { FunctionComponent, ReactElement, useState, useEffect } from 'react';
import React = require('react');
import { Cal, CalEntry } from '../types/CalEntry'

const CalData = require('../cal.json');

const Upcoming: FunctionComponent = (): ReactElement => {
    const [calEntries, setCalEntries] = useState<CalEntry[]>([]);

    useEffect(() => {
        const fetchCalData = async () => {
            try {
                const cacheBuster = new Date().getTime();
                const response = await fetch(`${CalData}?cb=${cacheBuster}`);
                if (!response.ok) throw new Error('Network response was not ok');
                const data: Cal = await response.json();
                setCalEntries(data.cal);
            } catch (error) {
                console.error("Fetching error:", error);
                setCalEntries([]);
            }
        };

        fetchCalData();
    }, []);

    var today = new Date();
    today.setHours(0, 0, 0, 0);

    let next3Entries = calEntries.filter((entry) => {
        let entryDate = new Date(entry.date);
        entryDate.setHours(0, 0, 0, 0);
        if (today < entryDate) {
            return true;
        }
        return false;
    }).slice(0, 3);

    let component = () => {
        if (next3Entries.length > 0) {
            return (
                <div className='next-three'>
                    <h1>Upcoming events</h1>
                    <table>
                        <tbody>
                            {next3Entries.map((entry, index) => (
                                <tr key={index}>
                                    <td className='date-column'>
                                        <h2>{entry.date} {new Date(entry.date).toLocaleDateString('en-US', { weekday: 'long' })}</h2>
                                    </td>
                                    <td className='event-column'>
                                        <h2>{entry.name}</h2>
                                    </td>
                                </tr>
                            ))}
                        </tbody>
                    </table>
                </div>
            );
        } else {
            return (
                <div className='next-three'>
                    <h1>No upcoming events</h1>
                </div>
            );
        }
    };

    return component();
};

export { Upcoming };
