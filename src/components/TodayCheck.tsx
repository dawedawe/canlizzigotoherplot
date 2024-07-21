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

    let today = new Date();
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
                <div>
                    <p>There might be traffic because of</p>
                    {
                        todaysEntries.map((entry, index) => (
                            <div key={index}>
                                <p>{entry.name} ({entry.date})</p>
                            </div>
                        ))
                    }
                </div>
            );
        } else {
            return (
                <p>Looks good. No stadium events today.</p>
            );
        }
    };

    return component();
};

export { TodayCheck };
