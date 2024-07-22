import { FunctionComponent, ReactElement, useState, useEffect } from 'react';
import React = require('react');
import { Cal, CalEntry } from '../types/CalEntry'

const CalData = require('../cal.json');

const TodayCheck: FunctionComponent = (): ReactElement => {
    const [calEntries, setCalEntries] = useState<CalEntry[]>([]);

    useEffect(() => {
        const fetchCalData = async () => {
            try {
                const response = await fetch(CalData);
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
    let todaysEntries = calEntries.filter((entry) => {
        let entryDate = new Date(entry.date);
        if (today.toDateString() === entryDate.toDateString()) {
            return true;
        }
        return false;
    });

    let component = () => {
        if (todaysEntries.length > 0) {
            return (
                <section className='traffic-today'>
                    <h2>There might be traffic :(</h2>
                    {
                        todaysEntries.map((entry, index) => (
                            <div key={index}>
                                <h3>{entry.name} ({entry.date})</h3>
                            </div>
                        ))
                    }
                </section>
            );
        } else {
            return (
                <div>
                    <h2>Looks good :)</h2>
                    <h3>No stadium events today</h3>
                </div>
            );
        }
    };

    return component();
};

export { TodayCheck };
