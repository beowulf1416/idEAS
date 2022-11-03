import { Component, OnInit } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { MessageType } from 'src/app/classes/message-type';
import { MessageService } from 'src/app/services/message.service';
import { TitleService } from 'src/app/services/title.service';
import { ProfileService } from '../../services/profile.service';

@Component({
  selector: 'app-general',
  templateUrl: './general.component.html',
  styleUrls: ['./general.component.css']
})
export class GeneralComponent implements OnInit {

  submitting = false;
  messages = '';

  profileForm = new FormGroup({
    people_id: new FormControl('', []),
    given_name: new FormControl('', [
      Validators.required
    ]),
    middle_name: new FormControl('', []),
    family_name: new FormControl('', [
      Validators.required
    ]),
    prefix: new FormControl('', []),
    suffix: new FormControl('', [])
  });

  constructor(
    private title: TitleService,
    private msg_service: MessageService,
    private profile_service: ProfileService
  ) {
    this.title.set_title("Profile");
  }

  ngOnInit(): void {
    this.msg_service.message$.subscribe(r => {
      this.messages = r.message;
    });

    this.profile_service.get().subscribe(r => {
      if (r.success) {
        const people = (r.data as { people: {
          people_id: string,
          given_name: string,
          middle_name: string,
          family_name: string,
          prefix: string,
          suffix: string
        } }).people;

        console.debug("GeneralComponent::ngOnInit()", people);
        if (people != null) {
          this.set_value(people);
        }
      } else {
        console.error("GeneralComponent::ngOnInit()", r);
        this.msg_service.send(r.message, MessageType.error);
      }
    });
  }

  get people_id() {
    return this.profileForm.get("people_id");
  }

  get given_name() {
    return this.profileForm.get("given_name");
  }

  get middle_name() {
    return this.profileForm.get("middle_name");
  }

  get family_name() {
    return this.profileForm.get("family_name");
  }

  get prefix() {
    return this.profileForm.get("prefix");
  }

  get suffix() {
    return this.profileForm.get("suffix");
  }

  set_value(people: {
    people_id: string,
    given_name: string,
    middle_name: string,
    family_name: string,
    prefix: string,
    suffix: string
  }) {
    this.people_id?.setValue(people.people_id);
    this.given_name?.setValue(people.given_name);
    this.middle_name?.setValue(people.middle_name);
    this.family_name?.setValue(people.family_name);
    this.prefix?.setValue(people.prefix);
    this.suffix?.setValue(people.suffix);
  }

  submit() {
    console.log("GeneralComponent::submit()");
    if (this.profileForm.valid) {
      this.submitting = true;
      this.profile_service.update(
        this.people_id?.value || '',
        this.given_name?.value || '',
        this.middle_name?.value || '',
        this.family_name?.value || '',
        this.prefix?.value || '',
        this.suffix?.value || ''
      ).subscribe(r => {
        if (!r.success) {
          this.msg_service.send(r.message, MessageType.error);
        }
        this.submitting = false;
      });
    }
  }
}
