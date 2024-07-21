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

    let next3Entries = calEntries.filter((entry) => {
        let entryDate = new Date(entry.date);
        entryDate.setHours(0, 0, 0, 0);
        if (today < entryDate) {
            console.log("today:", today, "entryDate:", entryDate);
            return true;
        }
        return false;
    }).slice(0, 3);

    let next3EntriesComponent = () => {
        if (next3Entries.length > 0) {
            return (
                <div className='next-three'>
                    <h2>Upcoming events</h2>
                    {
                        next3Entries.map((entry, index) => (
                            <div key={index}>
                                <p>{entry.name} ({entry.date})</p>
                            </div>
                        ))
                    }
                </div>
            );
        } else {
            return (
                <div className='next-three'>
                    <h2>No upcoming events</h2>
                </div>
            );
        }
    };

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
                    {next3EntriesComponent()}
                </div>
            );
        } else {
            return (
                <div>
                    <p>Looks good. No stadium events today.</p>
                    {next3EntriesComponent()}
                </div>
            );
        }
    };

    return component();
};

export { TodayCheck };
