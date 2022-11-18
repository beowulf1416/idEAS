import { Component, OnInit } from '@angular/core';
import { TitleService } from 'src/app/services/title.service';

@Component({
  selector: 'app-organization-list',
  templateUrl: './organization-list.component.html',
  styleUrls: ['./organization-list.component.css']
})
export class OrganizationListComponent implements OnInit {

  constructor(
    private title: TitleService
  ) { 
    this.title.set_title("Organizations");
  }

  ngOnInit(): void {
  }

}
