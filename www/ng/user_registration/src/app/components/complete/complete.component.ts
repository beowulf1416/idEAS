import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import { AuthService } from 'src/app/services/auth.service';
import { TitleService } from 'src/app/services/title.service';

@Component({
  selector: 'app-complete',
  templateUrl: './complete.component.html',
  styleUrls: ['./complete.component.css']
})
export class CompleteComponent implements OnInit {

  token = '';

  constructor(
    private title: TitleService,
    private route: ActivatedRoute,
    private auth_service: AuthService
  ) { }

  ngOnInit(): void {
    this.title.set_title("Sign Up - Completing");
    let token = this.route.snapshot.paramMap.get('token');
  }

  submit() {
    console.log("CompleteComponent::submit()");

    if (this.token != '') {
      // this.auth_service.complete
    }
  }
}
