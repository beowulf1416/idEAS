import { Time } from '@angular/common';
import { Component } from '@angular/core';
import { TitleService } from 'src/app/services/title.service';

import { v4 as uuidv4 } from 'uuid';

export interface TimeEntry {
  id: string,
  start: Date,
  end: Date
};

@Component({
  selector: 'app-timesheet',
  templateUrl: './timesheet.component.html',
  styleUrls: ['./timesheet.component.css']
})
export class TimesheetComponent {


  entries: Array<TimeEntry> = new Array<TimeEntry>();

  constructor(
    private title: TitleService
  ) {
    this.title.set_title("Timesheet");

    this.entries.push({
      id: uuidv4(),
      start: new Date('2022*11-15'),
      end: new Date('2022*11-15')
    });
  }
}
